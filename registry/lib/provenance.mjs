import { createPublicKey, verify } from 'node:crypto';
import { pae } from './signer.mjs';

export const PROVENANCE_PREDICATE = 'https://slsa.dev/provenance/v1';
export const CINCH_BUILD_TYPE = 'https://cinch.systems/buildtypes/cinch/v1';
export const PAYLOAD_TYPE = 'application/vnd.in-toto+json';
export const STATEMENT_TYPE = 'https://in-toto.io/Statement/v1';

function decodeStatement(envelope) {
  if (!envelope || typeof envelope !== 'object' || Array.isArray(envelope)) {
    return { error: 'DSSE envelope must be an object' };
  }
  if (envelope.payloadType !== PAYLOAD_TYPE) {
    return { error: `payloadType must be ${PAYLOAD_TYPE}` };
  }
  if (typeof envelope.payload !== 'string' || envelope.payload.length === 0) {
    return { error: 'DSSE payload must be a non-empty base64 string' };
  }
  if (!Array.isArray(envelope.signatures)) {
    return { error: 'DSSE signatures must be an array' };
  }
  let payload;
  let statement;
  try {
    payload = Buffer.from(envelope.payload, 'base64');
    statement = JSON.parse(payload.toString('utf8'));
  } catch {
    return { error: 'DSSE payload is not valid base64 JSON' };
  }
  return { payload, statement };
}

function subjectDigest(statement) {
  const subject = statement?.subject;
  if (!Array.isArray(subject) || subject.length === 0) return null;
  for (const item of subject) {
    const digest = item?.digest?.sha256;
    if (typeof digest === 'string' && /^[0-9a-f]{64}$/i.test(digest)) return digest.toLowerCase();
  }
  return null;
}

function verifySignatures(envelope, payload, trustedKeys) {
  if (envelope.signatures.length === 0) return { signed: false, trusted: false };
  let anyValid = false;
  let anyTrusted = false;
  for (const signature of envelope.signatures) {
    if (!signature || typeof signature.keyid !== 'string' || typeof signature.sig !== 'string') {
      return { signed: true, trusted: false, error: 'signature needs keyid and sig' };
    }
    const jwk = trustedKeys[signature.keyid];
    if (!jwk) continue;
    try {
      const key = createPublicKey({ key: jwk, format: 'jwk' });
      if (
        verify(
          'sha256',
          pae(envelope.payloadType, payload),
          { key, dsaEncoding: 'ieee-p1363' },
          Buffer.from(signature.sig, 'base64'),
        )
      ) {
        anyValid = true;
        anyTrusted = true;
      } else {
        return { signed: true, trusted: false, error: `signature ${signature.keyid} is invalid` };
      }
    } catch {
      return { signed: true, trusted: false, error: `trusted key ${signature.keyid} is invalid` };
    }
  }
  return {
    signed: true,
    trusted: anyTrusted,
    valid: anyValid,
    error: anyValid ? undefined : 'no signature matches a trusted builder key',
  };
}

/** Verify a provenance attestation against one release identity. */
export function verifyProvenance(envelope, packageId, trustedKeys = {}) {
  const decoded = decodeStatement(envelope);
  if (decoded.error) return { ok: false, reason: decoded.error };
  const { payload, statement } = decoded;
  if (statement?._type !== STATEMENT_TYPE) {
    return { ok: false, reason: `statement _type must be ${STATEMENT_TYPE}` };
  }
  if (statement.predicateType !== PROVENANCE_PREDICATE) {
    return { ok: false, reason: `predicateType must be ${PROVENANCE_PREDICATE}` };
  }
  const digest = subjectDigest(statement);
  const expected = packageId?.replace(/^sha256:/, '').toLowerCase();
  if (!digest || digest !== expected) {
    return { ok: false, reason: 'provenance subject digest does not match the release identity' };
  }
  const predicate = statement.predicate;
  const build = predicate?.buildDefinition;
  if (build?.buildType !== CINCH_BUILD_TYPE) {
    return { ok: false, reason: `buildType must be ${CINCH_BUILD_TYPE}` };
  }
  if (build.externalParameters?.package?.replace(/^sha256:/, '') !== expected) {
    return { ok: false, reason: 'externalParameters.package does not match the release identity' };
  }
  if (typeof predicate?.runDetails?.builder?.id !== 'string' || !predicate.runDetails.builder.id) {
    return { ok: false, reason: 'runDetails.builder.id is required' };
  }
  const signature = verifySignatures(envelope, payload, trustedKeys);
  if (signature.error && signature.signed && Object.keys(trustedKeys).length > 0) {
    return { ok: false, reason: signature.error };
  }
  return {
    ok: true,
    verified: signature.trusted,
    level: signature.trusted ? 'SLSA_BUILD_LEVEL_2' : 'SLSA_BUILD_LEVEL_1',
    verification: signature.trusted
      ? 'trusted-builder'
      : signature.signed
        ? 'untrusted-signature'
        : 'unsigned-l1',
    statement,
  };
}

export function isProvenanceType(predicateType) {
  return predicateType === PROVENANCE_PREDICATE;
}
