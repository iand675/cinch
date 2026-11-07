import { createPrivateKey, createPublicKey, generateKeyPairSync, sign, verify } from 'node:crypto';
import { chmod, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { canonicalDigest, canonicalJson, sha256Hex } from './canonical.mjs';

const PAYLOAD_TYPE = 'application/vnd.in-toto+json';

/** DSSE pre-authentication encoding: DSSEv1 <len(type)> <type> <len(payload)> <payload>. */
export function pae(payloadType, payload) {
  const prefix = Buffer.from(
    `DSSEv1 ${Buffer.byteLength(payloadType)} ${payloadType} ${payload.length} `,
    'utf8',
  );
  return Buffer.concat([prefix, payload]);
}

export async function openSigner(dataDir) {
  await mkdir(dataDir, { recursive: true });
  const keyFile = path.join(dataDir, 'registry-key.json');
  let keys;
  try {
    keys = JSON.parse(await readFile(keyFile, 'utf8'));
  } catch {
    const pair = generateKeyPairSync('ec', { namedCurve: 'prime256v1' });
    keys = {
      publicKey: pair.publicKey.export({ format: 'jwk' }),
      privateKey: pair.privateKey.export({ format: 'jwk' }),
    };
    await writeFile(keyFile, JSON.stringify(keys, null, 2), { mode: 0o600 });
    await chmod(keyFile, 0o600);
  }
  const privateKey = createPrivateKey({ key: keys.privateKey, format: 'jwk' });
  const publicKey = createPublicKey({ key: keys.publicKey, format: 'jwk' });
  const keyid = sha256Hex(canonicalJson(keys.publicKey));

  return {
    keyid,
    publicInfo() {
      return {
        keyid,
        algorithm: 'ES256',
        publicKey: keys.publicKey,
      };
    },
    signStatement(statement) {
      const payloadJson = canonicalJson(statement);
      const payload = Buffer.from(payloadJson, 'utf8');
      const signature = sign('sha256', pae(PAYLOAD_TYPE, payload), {
        key: privateKey,
        dsaEncoding: 'ieee-p1363',
      });
      const envelope = {
        payloadType: PAYLOAD_TYPE,
        payload: payload.toString('base64'),
        signatures: [{ keyid, sig: signature.toString('base64') }],
      };
      return {
        statement,
        payloadJson,
        envelope,
        digest: canonicalDigest(envelope),
      };
    },
    verifyEnvelope(envelope) {
      const signature = envelope.signatures?.[0];
      if (!signature) return false;
      const payload = Buffer.from(envelope.payload, 'base64');
      return verify(
        'sha256',
        pae(envelope.payloadType, payload),
        { key: publicKey, dsaEncoding: 'ieee-p1363' },
        Buffer.from(signature.sig, 'base64'),
      );
    },
  };
}
