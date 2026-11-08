import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash, createPublicKey, generateKeyPairSync, sign, verify } from 'node:crypto';
import { mkdtemp, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { createRegistry } from '../server.mjs';
import { canonicalJson, packageId, sha256Hex } from '../lib/canonical.mjs';
import { pae } from '../lib/signer.mjs';
import { CINCH_BUILD_TYPE, PAYLOAD_TYPE, PROVENANCE_PREDICATE, STATEMENT_TYPE } from '../lib/provenance.mjs';

const ADMIN = 'admin-test-token';

function makePackage(name, version, extraFiles = {}) {
  const contents = { 'Main.cinch': 'module Main where\n', ...extraFiles };
  const manifest = {
    schema: 2,
    name,
    version,
    modules: [{ name: 'Main', path: 'Main.cinch' }],
    order: ['Main'],
  };
  const files = Object.fromEntries(
    Object.entries(contents).map(([filePath, text]) => [
      filePath,
      Buffer.from(text, 'utf8').toString('base64'),
    ]),
  );
  return { manifest, files };
}

function computedId(pkg) {
  return packageId(
    pkg.manifest,
    Object.fromEntries(
      Object.entries(pkg.files).map(([filePath, encoded]) => [
        filePath,
        Buffer.from(encoded, 'base64'),
      ]),
    ),
  );
}

test('cinch registry v0: public and private packages', async (t) => {
  const dataDir = await mkdtemp(path.join(os.tmpdir(), 'cinch-registry-'));
  const registry = await createRegistry({ dataDir, adminToken: ADMIN, port: 0 });
  const base = registry.url;
  const state = {};

  t.after(async () => {
    await registry.close();
    await rm(dataDir, { recursive: true, force: true });
  });

  async function call(pathname, { method = 'GET', token, body } = {}) {
    const headers = {};
    if (token) headers.authorization = `Bearer ${token}`;
    if (body !== undefined) headers['content-type'] = 'application/json';
    const response = await fetch(base + pathname, {
      method,
      headers,
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    const text = await response.text();
    let json = null;
    try {
      json = text ? JSON.parse(text) : null;
    } catch {
      json = null;
    }
    return { status: response.status, headers: response.headers, text, json };
  }

  await t.test('health and signup', async () => {
    const health = await call('/health');
    assert.equal(health.status, 200);
    assert.equal(health.json.ok, true);

    const alice = await call('/api/v1/principals', {
      method: 'POST',
      body: { handle: 'alice' },
    });
    assert.equal(alice.status, 201);
    assert.equal(alice.json.principal.handle, 'alice');
    assert.match(alice.json.token, /^[0-9a-f]{64}$/);
    state.aliceToken = alice.json.token;

    const bob = await call('/api/v1/principals', {
      method: 'POST',
      body: { handle: 'bob' },
    });
    assert.equal(bob.status, 201);
    state.bobToken = bob.json.token;

    const org = await call('/api/v1/namespaces', {
      method: 'POST',
      token: state.aliceToken,
      body: { name: 'acme', kind: 'org' },
    });
    assert.equal(org.status, 201);
    assert.equal(org.json.name, 'acme');

    const member = await call('/api/v1/namespaces/acme/members', {
      method: 'POST',
      token: state.aliceToken,
      body: { handle: 'bob', role: 'reader' },
    });
    assert.equal(member.status, 200);

    const acmeToken = await call('/api/v1/tokens', {
      method: 'POST',
      token: state.aliceToken,
      body: { scopes: ['read', 'publish'], namespaces: ['acme'] },
    });
    assert.equal(acmeToken.status, 201);
    state.acmeToken = acmeToken.json.token;

    const bobReader = await call('/api/v1/tokens', {
      method: 'POST',
      token: state.bobToken,
      body: { scopes: ['read'], namespaces: ['acme'] },
    });
    assert.equal(bobReader.status, 201);
    state.bobAcmeToken = bobReader.json.token;
  });

  await t.test('publish and download a public package', async () => {
    const pkg = makePackage('alice/hello', '0.1.0');
    const expected = computedId(pkg);
    const published = await call('/api/v1/packages', {
      method: 'POST',
      token: state.aliceToken,
      body: { ...pkg, visibility: 'public', readme: 'Hello from Alice.' },
    });
    assert.equal(published.status, 201);
    assert.equal(published.json.entry.packageId, expected);
    state.hello = {
      pkg,
      pid: expected,
      hex: expected.slice('sha256:'.length),
      bundle: published.json.entry.bundle,
    };

    const index = await call('/api/v1/index');
    assert.equal(index.status, 200);
    assert.ok(index.json.packages.some((entry) => entry.name === 'alice/hello'));

    const download = await call(`/api/v1/download/${state.hello.hex}`);
    assert.equal(download.status, 200);
    assert.equal(download.json.manifest.name, 'alice/hello');
    assert.equal(download.json.files['Main.cinch'], pkg.files['Main.cinch']);
    assert.equal(
      `sha256:${createHash('sha256').update(download.text).digest('hex')}`,
      state.hello.bundle,
    );

    const home = await fetch(`${base}/`);
    assert.equal(home.status, 200);
    const homeHtml = await home.text();
    assert.match(homeHtml, /alice\/hello/);

    const page = await fetch(`${base}/packages/${encodeURIComponent('alice/hello')}`);
    assert.equal(page.status, 200);
    const pageHtml = await page.text();
    assert.match(pageHtml, /0\.1\.0/);
    assert.match(pageHtml, /Hello from Alice\./);
  });

  await t.test('private packages are scoped to namespace and token', async () => {
    const secret = makePackage('acme/secret', '0.1.0');
    const published = await call('/api/v1/packages', {
      method: 'POST',
      token: state.acmeToken,
      body: { ...secret, visibility: 'private', readme: 'Secret plans.' },
    });
    assert.equal(published.status, 201);
    state.secret = {
      pid: published.json.entry.packageId,
      hex: published.json.entry.packageId.slice('sha256:'.length),
    };

    const publicIndex = await call('/api/v1/index');
    assert.ok(!publicIndex.json.packages.some((entry) => entry.name === 'acme/secret'));

    const anonymousPrivate = await call('/api/v1/index/private');
    assert.equal(anonymousPrivate.status, 401);

    const bobIndex = await call('/api/v1/index/private', { token: state.bobAcmeToken });
    assert.equal(bobIndex.status, 200);
    assert.ok(bobIndex.json.packages.some((entry) => entry.name === 'acme/secret'));

    const aliceScopedIndex = await call('/api/v1/index/private', { token: state.aliceToken });
    assert.equal(aliceScopedIndex.status, 200);
    assert.ok(!aliceScopedIndex.json.packages.some((entry) => entry.name === 'acme/secret'));

    const adminIndex = await call('/api/v1/index/private', { token: ADMIN });
    assert.ok(adminIndex.json.packages.some((entry) => entry.name === 'acme/secret'));
    const adminSearch = await call('/api/v1/search?q=secret', { token: ADMIN });
    assert.ok(adminSearch.json.results.some((entry) => entry.name === 'acme/secret'));

    const anonymousMeta = await call('/api/v1/packages/acme%2Fsecret/0.1.0');
    assert.equal(anonymousMeta.status, 404);

    const bobMeta = await call('/api/v1/packages/acme%2Fsecret/0.1.0', {
      token: state.bobAcmeToken,
    });
    assert.equal(bobMeta.status, 200);
    assert.equal(bobMeta.json.visibility, 'private');

    const anonymousDownload = await call(`/api/v1/download/${state.secret.hex}`);
    assert.equal(anonymousDownload.status, 404);

    const bobDownload = await call(`/api/v1/download/${state.secret.hex}`, {
      token: state.bobAcmeToken,
    });
    assert.equal(bobDownload.status, 200);
    assert.equal(bobDownload.json.manifest.name, 'acme/secret');

    const bobPublish = await call('/api/v1/packages', {
      method: 'POST',
      token: state.bobAcmeToken,
      body: { ...makePackage('acme/other', '0.1.0'), visibility: 'private' },
    });
    assert.equal(bobPublish.status, 403);

    const anonymousSearch = await call('/api/v1/search?q=secret');
    assert.ok(!anonymousSearch.json.results.some((entry) => entry.name === 'acme/secret'));

    const bobSearch = await call('/api/v1/search?q=secret', { token: state.bobAcmeToken });
    assert.ok(bobSearch.json.results.some((entry) => entry.name === 'acme/secret'));

    const privatePage = await fetch(`${base}/packages/acme%2Fsecret`);
    assert.equal(privatePage.status, 404);
  });

  await t.test('rejects duplicate, mismatched, and unsafe publishes', async () => {
    const duplicate = await call('/api/v1/packages', {
      method: 'POST',
      token: state.aliceToken,
      body: { ...state.hello.pkg, visibility: 'public' },
    });
    assert.equal(duplicate.status, 409);

    const mismatched = await call('/api/v1/packages', {
      method: 'POST',
      token: state.aliceToken,
      body: {
        ...makePackage('alice/mismatch', '0.1.0'),
        packageId: `sha256:${'0'.repeat(64)}`,
      },
    });
    assert.equal(mismatched.status, 400);

    const traversal = makePackage('alice/traversal', '0.1.0');
    traversal.files['../evil.cinch'] = Buffer.from('x').toString('base64');
    const unsafe = await call('/api/v1/packages', {
      method: 'POST',
      token: state.aliceToken,
      body: traversal,
    });
    assert.equal(unsafe.status, 400);

    const missing = makePackage('alice/missing', '0.1.0');
    missing.manifest.modules = [{ name: 'Other', path: 'Other.cinch' }];
    const missingModule = await call('/api/v1/packages', {
      method: 'POST',
      token: state.aliceToken,
      body: missing,
    });
    assert.equal(missingModule.status, 400);
  });

  await t.test('yank keeps the artifact and marks the index', async () => {
    const yank = await call('/api/v1/packages/alice%2Fhello/0.1.0/yank', {
      method: 'POST',
      token: state.aliceToken,
      body: { reason: 'superseded' },
    });
    assert.equal(yank.status, 200);

    const index = await call('/api/v1/index');
    const entry = index.json.packages.find((item) => item.name === 'alice/hello');
    assert.equal(entry.yanked, true);

    const download = await call(`/api/v1/download/${state.hello.hex}`);
    assert.equal(download.status, 200);

    const statements = await call('/api/v1/statements');
    assert.ok(
      statements.json.statements.some(
        (item) => item.kind === 'yank' && item.name === 'alice/hello',
      ),
    );
  });

  await t.test('takedown blocks, delists, and signs the statement', async () => {
    const takedown = await call('/api/v1/packages/alice%2Fhello/0.1.0/takedown', {
      method: 'POST',
      token: ADMIN,
      body: { reason: 'copyright' },
    });
    assert.equal(takedown.status, 200);

    const download = await call(`/api/v1/download/${state.hello.hex}`);
    assert.equal(download.status, 410);

    const index = await call('/api/v1/index');
    assert.ok(!index.json.packages.some((entry) => entry.name === 'alice/hello'));

    const statements = await call('/api/v1/statements');
    const event = statements.json.statements.find(
      (item) => item.kind === 'takedown' && item.name === 'alice/hello',
    );
    assert.ok(event);
    assert.equal(event.envelope.payloadType, 'application/vnd.in-toto+json');

    const key = await call('/api/v1/registry-key');
    assert.equal(key.status, 200);
    const publicKey = createPublicKey({ key: key.json.publicKey, format: 'jwk' });
    const payload = Buffer.from(event.envelope.payload, 'base64');
    const signature = Buffer.from(event.envelope.signatures[0].sig, 'base64');
    assert.equal(
      verify(
        'sha256',
        pae(event.envelope.payloadType, payload),
        { key: publicKey, dsaEncoding: 'ieee-p1363' },
        signature,
      ),
      true,
    );
    const tampered = Buffer.from(payload);
    tampered[0] ^= 1;
    assert.equal(
      verify(
        'sha256',
        pae(event.envelope.payloadType, tampered),
        { key: publicKey, dsaEncoding: 'ieee-p1363' },
        signature,
      ),
      false,
    );
  });

  await t.test('restore unblocks the release', async () => {
    const restore = await call('/api/v1/packages/alice%2Fhello/0.1.0/restore', {
      method: 'POST',
      token: ADMIN,
      body: { reason: 'counter-notice accepted' },
    });
    assert.equal(restore.status, 200);
    const download = await call(`/api/v1/download/${state.hello.hex}`);
    assert.equal(download.status, 200);
    const index = await call('/api/v1/index');
    assert.ok(index.json.packages.some((entry) => entry.name === 'alice/hello'));
  });

  await t.test('token revocation takes effect', async () => {
    const temp = await call('/api/v1/tokens', {
      method: 'POST',
      token: state.aliceToken,
      body: { scopes: ['read'], namespaces: ['alice'] },
    });
    assert.equal(temp.status, 201);
    const before = await call('/api/v1/index/private', { token: temp.json.token });
    assert.equal(before.status, 200);
    const deleted = await call(`/api/v1/tokens/${temp.json.id}`, {
      method: 'DELETE',
      token: state.aliceToken,
    });
    assert.equal(deleted.status, 204);
    const after = await call('/api/v1/index/private', { token: temp.json.token });
    assert.equal(after.status, 401);
  });

  await t.test('only an owner manages members', async () => {
    const attempt = await call('/api/v1/namespaces/acme/members', {
      method: 'POST',
      token: state.bobAcmeToken,
      body: { handle: 'alice', role: 'reader' },
    });
    assert.equal(attempt.status, 403);
  });
});


function provenanceStatement(pkg, pid) {
  const hex = pid.slice('sha256:'.length);
  return {
    _type: STATEMENT_TYPE,
    subject: [{ name: pkg.manifest.name, digest: { sha256: hex } }],
    predicateType: PROVENANCE_PREDICATE,
    predicate: {
      buildDefinition: {
        buildType: CINCH_BUILD_TYPE,
        externalParameters: { package: pid, profile: 'native', stageMode: 'refresh' },
        resolvedDependencies: [],
      },
      runDetails: { builder: { id: 'local:cinch' }, metadata: {} },
    },
  };
}

function provenanceEnvelope(pkg, pid, key) {
  const payload = Buffer.from(JSON.stringify(provenanceStatement(pkg, pid)), 'utf8');
  const signatures = [];
  if (key) {
    signatures.push({
      keyid: key.keyid,
      sig: sign('sha256', pae(PAYLOAD_TYPE, payload), {
        key: key.privateKey,
        dsaEncoding: 'ieee-p1363',
      }).toString('base64'),
    });
  }
  return {
    payloadType: PAYLOAD_TYPE,
    payload: payload.toString('base64'),
    signatures,
  };
}

test('cinch registry verifies provenance attestations', async (t) => {
  const keyPair = generateKeyPairSync('ec', { namedCurve: 'prime256v1' });
  const publicJwk = keyPair.publicKey.export({ format: 'jwk' });
  const keyid = sha256Hex(canonicalJson(publicJwk));
  const trustedKey = {
    keyid,
    privateKey: keyPair.privateKey,
    publicKey: publicJwk,
  };
  const dataDir = await mkdtemp(path.join(os.tmpdir(), 'cinch-provenance-'));
  const registry = await createRegistry({
    dataDir,
    adminToken: ADMIN,
    provenanceKeys: { [keyid]: publicJwk },
    port: 0,
  });
  t.after(async () => {
    await registry.close();
    await rm(dataDir, { recursive: true, force: true });
  });
  const base = registry.url;
  async function call(pathname, { method = 'GET', token, body } = {}) {
    const headers = {};
    if (token) headers.authorization = `Bearer ${token}`;
    if (body !== undefined) headers['content-type'] = 'application/json';
    const response = await fetch(base + pathname, {
      method,
      headers,
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    const text = await response.text();
    return { status: response.status, json: text ? JSON.parse(text) : null };
  }
  const alice = await call('/api/v1/principals', {
    method: 'POST',
    body: { handle: 'alice' },
  });
  const token = alice.json.token;

  const unsigned = makePackage('alice/unsigned', '0.1.0');
  const unsignedId = computedId(unsigned);
  const unsignedPublish = await call('/api/v1/packages', {
    method: 'POST',
    token,
    body: {
      ...unsigned,
      visibility: 'public',
      attestations: [{
        predicateType: PROVENANCE_PREDICATE,
        envelope: provenanceEnvelope(unsigned, unsignedId),
      }],
    },
  });
  assert.equal(unsignedPublish.status, 201);
  const unsignedMeta = await call('/api/v1/packages/alice%2Funsigned/0.1.0');
  assert.equal(unsignedMeta.status, 200);
  assert.equal(unsignedMeta.json.attestations[0].verified, false);
  assert.equal(unsignedMeta.json.attestations[0].verification, 'unsigned-l1');
  assert.equal(unsignedMeta.json.attestations[0].level, 'SLSA_BUILD_LEVEL_1');

  const signed = makePackage('alice/signed', '0.1.0');
  const signedId = computedId(signed);
  const signedPublish = await call('/api/v1/packages', {
    method: 'POST',
    token,
    body: {
      ...signed,
      visibility: 'public',
      attestations: [{
        predicateType: PROVENANCE_PREDICATE,
        envelope: provenanceEnvelope(signed, signedId, trustedKey),
      }],
    },
  });
  assert.equal(signedPublish.status, 201);
  const signedMeta = await call('/api/v1/packages/alice%2Fsigned/0.1.0');
  assert.equal(signedMeta.status, 200);
  assert.equal(signedMeta.json.attestations[0].verified, true);
  assert.equal(signedMeta.json.attestations[0].verification, 'trusted-builder');
  assert.equal(signedMeta.json.attestations[0].level, 'SLSA_BUILD_LEVEL_2');

  const wrong = makePackage('alice/wrong', '0.1.0');
  const wrongPublish = await call('/api/v1/packages', {
    method: 'POST',
    token,
    body: {
      ...wrong,
      visibility: 'public',
      attestations: [{
        predicateType: PROVENANCE_PREDICATE,
        envelope: provenanceEnvelope(wrong, computedId(signed)),
      }],
    },
  });
  assert.equal(wrongPublish.status, 400);
});
