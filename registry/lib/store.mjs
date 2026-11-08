import { DatabaseSync } from 'node:sqlite';
import { mkdir, readFile, rename, writeFile } from 'node:fs/promises';
import path from 'node:path';
import {
  canonicalDigest,
  canonicalJson,
  newToken,
  packageId,
  randomId,
  sha256Hex,
  tokenHash,
} from './canonical.mjs';
import { badRequest, conflict, gone, notFound, unauthorized } from './errors.mjs';
import { PROVENANCE_PREDICATE, isProvenanceType, verifyProvenance } from './provenance.mjs';

const MAX_FILE_BYTES = 24 * 1024 * 1024;
const MAX_README = 64 * 1024;
const RESERVED = new Set([
  'admin',
  'api',
  'www',
  'cinch',
  'support',
  'security',
  'registry',
  'packages',
  'download',
  'docs',
]);

const now = () => new Date().toISOString();

function validateHandle(value, what = 'handle') {
  if (typeof value !== 'string' || !/^[a-z0-9][a-z0-9_-]{1,38}$/.test(value)) {
    throw badRequest(`${what} must be 2 to 39 lowercase letters, digits, "_", or "-"`);
  }
  if (RESERVED.has(value)) throw badRequest(`${what} "${value}" is reserved`);
  return value;
}

function splitPackageName(value) {
  if (typeof value !== 'string') throw badRequest('package name must be a string');
  const parts = value.split('/');
  if (parts.length !== 2) throw badRequest('package name must be namespace/name');
  const [namespace, name] = parts;
  validateHandle(namespace, 'namespace');
  if (!/^[a-z0-9][a-z0-9._-]{0,63}$/.test(name) || name.includes('..')) {
    throw badRequest('package name is invalid');
  }
  return { namespace, name };
}

function validateVersion(value) {
  if (typeof value !== 'string' || !/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(value)) {
    throw badRequest('version must be major.minor.patch with an optional prerelease');
  }
  return value;
}

function safePath(value) {
  if (typeof value !== 'string' || value.length === 0 || value.length > 512) {
    throw badRequest('file path is invalid');
  }
  if (value.startsWith('/') || value.includes('\\') || value.includes('\0')) {
    throw badRequest(`file path "${value}" is not a relative POSIX path`);
  }
  const segments = value.split('/');
  if (segments.some((s) => s === '' || s === '.' || s === '..')) {
    throw badRequest(`file path "${value}" contains an empty, "." or ".." segment`);
  }
  return value;
}

function decodeBase64(value, label) {
  if (typeof value !== 'string' || value.length % 4 !== 0 || !/^[A-Za-z0-9+/]*={0,2}$/.test(value)) {
    throw badRequest(`${label} is not valid base64`);
  }
  return Buffer.from(value, 'base64');
}

function validateFiles(input) {
  if (!input || typeof input !== 'object' || Array.isArray(input)) {
    throw badRequest('files must be an object of path to base64 contents');
  }
  const files = {};
  let total = 0;
  for (const [rawPath, encoded] of Object.entries(input)) {
    const filePath = safePath(rawPath);
    const bytes = Buffer.isBuffer(encoded) ? encoded : decodeBase64(encoded, `file "${filePath}"`);
    total += bytes.length;
    if (total > MAX_FILE_BYTES) throw badRequest('package files exceed the 24 MiB limit');
    files[filePath] = bytes;
  }
  if (Object.keys(files).length === 0) throw badRequest('a package needs at least one file');
  return files;
}

function validateManifest(input, files) {
  if (!input || typeof input !== 'object' || Array.isArray(input)) {
    throw badRequest('manifest must be an object');
  }
  if (input.schema !== 1 && input.schema !== 2) {
    throw badRequest('manifest schema must be 1 or 2');
  }
  splitPackageName(input.name);
  validateVersion(input.version);
  if (!Array.isArray(input.modules) || input.modules.length === 0) {
    throw badRequest('manifest modules must be a non-empty array');
  }
  if (!Array.isArray(input.order)) throw badRequest('manifest order must be an array');
  const names = new Set();
  const paths = new Set();
  for (const module of input.modules) {
    if (!module || typeof module !== 'object') throw badRequest('each module must be an object');
    if (typeof module.name !== 'string' || module.name.length === 0) {
      throw badRequest('each module needs a name');
    }
    if (typeof module.path !== 'string') throw badRequest('each module needs a path');
    const filePath = safePath(module.path);
    if (!(filePath in files)) throw badRequest(`module ${module.name} has no file at ${filePath}`);
    if (names.has(module.name)) throw badRequest(`duplicate module ${module.name}`);
    if (paths.has(filePath)) throw badRequest(`duplicate module path ${filePath}`);
    names.add(module.name);
    paths.add(filePath);
  }
  const ordered = new Set();
  for (const name of input.order) {
    if (typeof name !== 'string') throw badRequest('order entries must be strings');
    if (!names.has(name)) throw badRequest(`order names unknown module ${name}`);
    if (ordered.has(name)) throw badRequest(`order names module ${name} twice`);
    ordered.add(name);
  }
  if (ordered.size !== names.size) throw badRequest('order must name every module exactly once');
  return input;
}

function validateAttestations(input, packageId, trustedKeys) {
  if (input === undefined || input === null) return [];
  if (!Array.isArray(input)) throw badRequest('attestations must be an array');
  if (input.length > 32) throw badRequest('at most 32 attestations per release');
  return input.map((item, index) => {
    if (!item || typeof item !== 'object') throw badRequest(`attestation ${index} must be an object`);
    if (typeof item.predicateType !== 'string' || item.predicateType.length === 0) {
      throw badRequest(`attestation ${index} needs a predicateType`);
    }
    if (!item.envelope || typeof item.envelope !== 'object') {
      throw badRequest(`attestation ${index} needs a DSSE envelope`);
    }
    const digest = canonicalDigest(item.envelope);
    let verification = null;
    if (isProvenanceType(item.predicateType)) {
      verification = verifyProvenance(item.envelope, packageId, trustedKeys);
      if (!verification.ok) {
        throw badRequest(`attestation ${index}: ${verification.reason}`);
      }
    }
    return {
      predicateType: item.predicateType,
      envelope: item.envelope,
      digest,
      verified: verification?.verified ? 1 : 0,
      level: verification?.level ?? null,
      verification: verification?.verification ?? null,
    };
  });
}

function blobPath(dataDir, digest) {
  const hex = digest.replace(/^sha256:/, '');
  return path.join(dataDir, 'blobs', hex.slice(0, 2), hex.slice(2));
}

function entry(row, includeReadme = false) {
  const value = {
    name: row.name,
    version: row.version,
    packageId: row.package_id,
    bundle: row.bundle_digest,
    visibility: row.visibility,
    namespace: row.namespace,
    publisher: row.publisher_id,
    yanked: Boolean(row.yanked),
    blocked: Boolean(row.blocked),
    sequence: row.sequence,
    createdAt: row.created_at,
  };
  if (includeReadme) value.readme = row.readme ?? null;
  return value;
}

function likePattern(text) {
  return `%${text.replace(/[\\%_]/g, '\\$&')}%`;
}

export async function openStore(dataDir, { adminToken, signer, provenanceKeys = {} }) {
  await mkdir(path.join(dataDir, 'blobs'), { recursive: true });
  const db = new DatabaseSync(path.join(dataDir, 'registry.db'));
  db.exec(`
    PRAGMA journal_mode = WAL;
    CREATE TABLE IF NOT EXISTS principals (
      id TEXT PRIMARY KEY,
      handle TEXT NOT NULL UNIQUE,
      kind TEXT NOT NULL,
      created_at TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS namespaces (
      name TEXT PRIMARY KEY,
      kind TEXT NOT NULL,
      owner_id TEXT NOT NULL,
      created_at TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS memberships (
      namespace TEXT NOT NULL,
      principal_id TEXT NOT NULL,
      role TEXT NOT NULL,
      PRIMARY KEY (namespace, principal_id)
    );
    CREATE TABLE IF NOT EXISTS tokens (
      id TEXT PRIMARY KEY,
      token_hash TEXT NOT NULL UNIQUE,
      principal_id TEXT NOT NULL,
      scopes TEXT NOT NULL,
      namespaces TEXT NOT NULL,
      created_at TEXT NOT NULL,
      expires_at TEXT
    );
    CREATE TABLE IF NOT EXISTS packages (
      name TEXT NOT NULL,
      version TEXT NOT NULL,
      package_id TEXT NOT NULL UNIQUE,
      bundle_digest TEXT NOT NULL,
      manifest_json TEXT NOT NULL,
      files_json TEXT NOT NULL,
      visibility TEXT NOT NULL,
      namespace TEXT NOT NULL,
      publisher_id TEXT NOT NULL,
      readme TEXT,
      yanked INTEGER NOT NULL DEFAULT 0,
      blocked INTEGER NOT NULL DEFAULT 0,
      block_reason TEXT,
      sequence INTEGER NOT NULL,
      created_at TEXT NOT NULL,
      PRIMARY KEY (name, version)
    );
    CREATE TABLE IF NOT EXISTS attestations (
      package_id TEXT NOT NULL,
      predicate_type TEXT NOT NULL,
      envelope_json TEXT NOT NULL,
      digest TEXT NOT NULL,
      verified INTEGER NOT NULL DEFAULT 0,
      level TEXT,
      verification TEXT,
      PRIMARY KEY (package_id, digest)
    );
    CREATE TABLE IF NOT EXISTS statements (
      seq INTEGER PRIMARY KEY AUTOINCREMENT,
      kind TEXT NOT NULL,
      name TEXT NOT NULL,
      version TEXT,
      reason TEXT NOT NULL,
      issuer TEXT NOT NULL,
      created_at TEXT NOT NULL,
      payload_json TEXT NOT NULL,
      envelope_json TEXT NOT NULL,
      digest TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS meta (
      key TEXT PRIMARY KEY,
      value TEXT NOT NULL
    );
    INSERT OR IGNORE INTO meta(key, value) VALUES ('index_sequence', '0');
  `);
  const attestationColumns = db.prepare('PRAGMA table_info(attestations)').all().map((row) => row.name);
  if (!attestationColumns.includes('level')) db.exec('ALTER TABLE attestations ADD COLUMN level TEXT');
  if (!attestationColumns.includes('verification')) db.exec('ALTER TABLE attestations ADD COLUMN verification TEXT');

  const store = {
    db,
    dataDir,
    signer,

    async writeBlob(bytes) {
      const digest = `sha256:${sha256Hex(bytes)}`;
      const file = blobPath(dataDir, digest);
      await mkdir(path.dirname(file), { recursive: true });
      try {
        await readFile(file);
        return digest;
      } catch {
        const tmp = `${file}.${process.pid}.${Date.now()}.tmp`;
        await writeFile(tmp, bytes);
        await rename(tmp, file);
        return digest;
      }
    },

    async readBlob(digest) {
      return readFile(blobPath(dataDir, digest));
    },

    nextSequence() {
      const row = db.prepare('SELECT value FROM meta WHERE key = ?').get('index_sequence');
      const next = Number(row?.value ?? 0) + 1;
      db.prepare(
        'INSERT INTO meta(key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value',
      ).run('index_sequence', String(next));
      return next;
    },

    indexSequence() {
      const row = db.prepare('SELECT value FROM meta WHERE key = ?').get('index_sequence');
      return Number(row?.value ?? 0);
    },

    membershipsFor(principalId) {
      return Object.fromEntries(
        db
          .prepare('SELECT namespace, role FROM memberships WHERE principal_id = ?')
          .all(principalId)
          .map((row) => [row.namespace, row.role]),
      );
    },

    ensureAdmin() {
      let admin = db.prepare('SELECT * FROM principals WHERE handle = ?').get('admin');
      if (!admin) {
        const id = randomId('p');
        db.prepare('INSERT INTO principals(id, handle, kind, created_at) VALUES (?, ?, ?, ?)').run(
          id,
          'admin',
          'user',
          now(),
        );
        db.prepare(
          'INSERT OR IGNORE INTO namespaces(name, kind, owner_id, created_at) VALUES (?, ?, ?, ?)',
        ).run('admin', 'user', id, now());
        db.prepare(
          'INSERT OR IGNORE INTO memberships(namespace, principal_id, role) VALUES (?, ?, ?)',
        ).run('admin', id, 'owner');
        admin = { id, handle: 'admin', kind: 'user' };
      }
      db.prepare('DELETE FROM tokens WHERE principal_id = ?').run(admin.id);
      db.prepare(
        'INSERT INTO tokens(id, token_hash, principal_id, scopes, namespaces, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?, ?)',
      ).run(
        randomId('t'),
        tokenHash(adminToken),
        admin.id,
        JSON.stringify(['read', 'publish', 'admin']),
        JSON.stringify(['*']),
        now(),
        null,
      );
      return admin.id;
    },

    findToken(raw) {
      if (!raw) return null;
      const row = db
        .prepare(
          `SELECT t.id, t.principal_id, t.scopes, t.namespaces, t.expires_at,
                  p.handle, p.kind
             FROM tokens t JOIN principals p ON p.id = t.principal_id
            WHERE t.token_hash = ?`,
        )
        .get(tokenHash(raw));
      if (!row) return null;
      if (row.expires_at && Date.parse(row.expires_at) <= Date.now()) return null;
      const memberships = db
        .prepare('SELECT namespace, role FROM memberships WHERE principal_id = ?')
        .all(row.principal_id);
      return {
        id: row.id,
        principal: { id: row.principal_id, handle: row.handle, kind: row.kind },
        scopes: JSON.parse(row.scopes),
        namespaces: JSON.parse(row.namespaces),
        memberships: Object.fromEntries(memberships.map((m) => [m.namespace, m.role])),
      };
    },

    createUser(rawHandle) {
      const handle = validateHandle(rawHandle);
      if (db.prepare('SELECT id FROM principals WHERE handle = ?').get(handle)) {
        throw conflict(`handle "${handle}" is already taken`);
      }
      const id = randomId('p');
      const token = newToken();
      db.exec('BEGIN');
      try {
        db.prepare('INSERT INTO principals(id, handle, kind, created_at) VALUES (?, ?, ?, ?)').run(
          id,
          handle,
          'user',
          now(),
        );
        db.prepare(
          'INSERT INTO namespaces(name, kind, owner_id, created_at) VALUES (?, ?, ?, ?)',
        ).run(handle, 'user', id, now());
        db.prepare(
          'INSERT INTO memberships(namespace, principal_id, role) VALUES (?, ?, ?)',
        ).run(handle, id, 'owner');
        db.prepare(
          'INSERT INTO tokens(id, token_hash, principal_id, scopes, namespaces, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?, ?)',
        ).run(
          randomId('t'),
          tokenHash(token),
          id,
          JSON.stringify(['read', 'publish']),
          JSON.stringify([handle]),
          now(),
          null,
        );
        db.exec('COMMIT');
      } catch (error) {
        db.exec('ROLLBACK');
        throw error;
      }
      return { principal: { id, handle, kind: 'user' }, token };
    },

    createOrg(rawName, ownerId) {
      const name = validateHandle(rawName, 'namespace');
      if (db.prepare('SELECT name FROM namespaces WHERE name = ?').get(name)) {
        throw conflict(`namespace "${name}" already exists`);
      }
      db.prepare('INSERT INTO namespaces(name, kind, owner_id, created_at) VALUES (?, ?, ?, ?)').run(
        name,
        'org',
        ownerId,
        now(),
      );
      db.prepare(
        'INSERT INTO memberships(namespace, principal_id, role) VALUES (?, ?, ?)',
      ).run(name, ownerId, 'owner');
      return { name, kind: 'org', owner: ownerId };
    },

    namespace(name) {
      return db.prepare('SELECT * FROM namespaces WHERE name = ?').get(name) ?? null;
    },

    principalByHandle(handle) {
      return db.prepare('SELECT * FROM principals WHERE handle = ?').get(handle) ?? null;
    },

    addMember(namespace, principalId, role) {
      if (!['owner', 'publisher', 'reader'].includes(role)) {
        throw badRequest('role must be owner, publisher, or reader');
      }
      if (!db.prepare('SELECT name FROM namespaces WHERE name = ?').get(namespace)) {
        throw notFound(`namespace "${namespace}" does not exist`);
      }
      db.prepare(
        `INSERT INTO memberships(namespace, principal_id, role) VALUES (?, ?, ?)
         ON CONFLICT(namespace, principal_id) DO UPDATE SET role = excluded.role`,
      ).run(namespace, principalId, role);
      return { namespace, principal: principalId, role };
    },

    createToken(principalId, scopes, namespaces, expiresAt = null) {
      const validScopes = ['read', 'publish', 'admin'];
      if (!Array.isArray(scopes) || scopes.length === 0 || scopes.some((s) => !validScopes.includes(s))) {
        throw badRequest('scopes must be a non-empty subset of read, publish, admin');
      }
      if (!Array.isArray(namespaces) || namespaces.length === 0) {
        throw badRequest('namespaces must be a non-empty array');
      }
      if (
        expiresAt !== null &&
        (typeof expiresAt !== 'string' || Number.isNaN(Date.parse(expiresAt)))
      ) {
        throw badRequest('expiresAt must be an ISO timestamp or null');
      }
      for (const ns of namespaces) {
        if (ns !== '*' && !db.prepare('SELECT name FROM namespaces WHERE name = ?').get(ns)) {
          throw notFound(`namespace "${ns}" does not exist`);
        }
      }
      const token = newToken();
      const id = randomId('t');
      db.prepare(
        'INSERT INTO tokens(id, token_hash, principal_id, scopes, namespaces, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?, ?)',
      ).run(
        id,
        tokenHash(token),
        principalId,
        JSON.stringify(scopes),
        JSON.stringify(namespaces),
        now(),
        expiresAt,
      );
      return { id, token, scopes, namespaces, expiresAt };
    },

    revokeToken(id, principalId, isAdmin) {
      const row = db.prepare('SELECT principal_id FROM tokens WHERE id = ?').get(id);
      if (!row) throw notFound('token does not exist');
      if (!isAdmin && row.principal_id !== principalId) throw unauthorized('not your token');
      db.prepare('DELETE FROM tokens WHERE id = ?').run(id);
    },

    async publish({ manifest, files, visibility, readme, attestations, publisherId, claimedPackageId }) {
      if (visibility !== 'public' && visibility !== 'private') {
        throw badRequest('visibility must be public or private');
      }
      if (readme !== undefined && readme !== null) {
        if (typeof readme !== 'string' || readme.length > MAX_README) {
          throw badRequest('readme must be a string of at most 64 KiB');
        }
      }
      const validatedFiles = validateFiles(files);
      const validatedManifest = validateManifest(manifest, validatedFiles);
      const { namespace } = splitPackageName(validatedManifest.name);
      if (!db.prepare('SELECT name FROM namespaces WHERE name = ?').get(namespace)) {
        throw notFound(`namespace "${namespace}" does not exist`);
      }
      if (db.prepare('SELECT 1 FROM packages WHERE name = ? AND version = ?').get(
        validatedManifest.name,
        validatedManifest.version,
      )) {
        throw conflict(`${validatedManifest.name}@${validatedManifest.version} already exists`);
      }
      const pid = packageId(validatedManifest, validatedFiles);
      if (claimedPackageId && claimedPackageId !== pid) {
        throw badRequest(`package identity mismatch: server computed ${pid}`);
      }
      if (db.prepare('SELECT 1 FROM packages WHERE package_id = ?').get(pid)) {
        throw conflict(`package identity ${pid} already exists`);
      }
      const attestationRecords = validateAttestations(attestations, pid, provenanceKeys);
      const encodedFiles = Object.fromEntries(
        Object.entries(validatedFiles).map(([filePath, bytes]) => [
          filePath,
          bytes.toString('base64'),
        ]),
      );
      const bundle = {
        schema: 1,
        manifest: validatedManifest,
        files: encodedFiles,
        visibility,
        readme: readme ?? null,
      };
      const bundleDigest = await store.writeBlob(Buffer.from(canonicalJson(bundle), 'utf8'));
      const sequence = store.nextSequence();
      db.exec('BEGIN');
      try {
        db.prepare(
          `INSERT INTO packages(
             name, version, package_id, bundle_digest, manifest_json, files_json,
             visibility, namespace, publisher_id, readme, yanked, blocked,
             block_reason, sequence, created_at
           ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, NULL, ?, ?)`,
        ).run(
          validatedManifest.name,
          validatedManifest.version,
          pid,
          bundleDigest,
          canonicalJson(validatedManifest),
          canonicalJson(
            Object.fromEntries(
              Object.entries(validatedFiles).map(([filePath, bytes]) => [filePath, sha256Hex(bytes)]),
            ),
          ),
          visibility,
          namespace,
          publisherId,
          readme ?? null,
          sequence,
          now(),
        );
        for (const item of attestationRecords) {
          db.prepare(
            'INSERT INTO attestations(package_id, predicate_type, envelope_json, digest, verified, level, verification) VALUES (?, ?, ?, ?, ?, ?, ?)',
          ).run(pid, item.predicateType, canonicalJson(item.envelope), item.digest, item.verified, item.level, item.verification);
        }
        db.exec('COMMIT');
      } catch (error) {
        db.exec('ROLLBACK');
        throw error;
      }
      const row = db.prepare('SELECT * FROM packages WHERE name = ? AND version = ?').get(
        validatedManifest.name,
        validatedManifest.version,
      );
      return { entry: entry(row), sequence };
    },

    getVersion(name, version) {
      return db.prepare('SELECT * FROM packages WHERE name = ? AND version = ?').get(name, version) ?? null;
    },

    getVersions(name) {
      return db.prepare('SELECT * FROM packages WHERE name = ? ORDER BY created_at DESC').all(name);
    },

    getByPackageId(pid) {
      return db.prepare('SELECT * FROM packages WHERE package_id = ?').get(pid) ?? null;
    },

    attestations(pid) {
      return db
        .prepare('SELECT predicate_type, envelope_json, digest, verified, level, verification FROM attestations WHERE package_id = ?')
        .all(pid)
        .map((row) => ({
          predicateType: row.predicate_type,
          envelope: JSON.parse(row.envelope_json),
          digest: row.digest,
          verified: Boolean(row.verified),
          level: row.predicate_type === PROVENANCE_PREDICATE
            ? row.level ?? (row.verified ? 'SLSA_BUILD_LEVEL_2' : 'SLSA_BUILD_LEVEL_1')
            : null,
          verification: row.predicate_type === PROVENANCE_PREDICATE
            ? row.verification ?? (row.verified ? 'trusted-builder' : 'unsigned-l1')
            : null,
        }));
    },

    async bundle(row) {
      return JSON.parse((await store.readBlob(row.bundle_digest)).toString('utf8'));
    },

    publicIndex(since = 0) {
      return db
        .prepare(
          `SELECT * FROM packages
            WHERE visibility = 'public' AND blocked = 0 AND sequence > ?
            ORDER BY name, version`,
        )
        .all(since)
        .map((row) => entry(row));
    },

    privateIndex(principalId, since = 0, all = false) {
      if (all) {
        return db
          .prepare(
            `SELECT * FROM packages
              WHERE blocked = 0 AND sequence > ?
              ORDER BY name, version`,
          )
          .all(since)
          .map((row) => entry(row));
      }
      return db
        .prepare(
          `SELECT * FROM packages p
            WHERE p.blocked = 0 AND p.sequence > ?
              AND (p.visibility = 'public'
                   OR EXISTS (SELECT 1 FROM memberships m
                               WHERE m.namespace = p.namespace AND m.principal_id = ?))
            ORDER BY p.name, p.version`,
        )
        .all(since, principalId)
        .map((row) => entry(row));
    },

    search(query, principalId = null, limit = 50, all = false) {
      const pattern = likePattern(query);
      if (all) {
        return db
          .prepare(
            `SELECT * FROM packages p
              WHERE p.blocked = 0
                AND (p.name LIKE ? ESCAPE '\\' OR p.readme LIKE ? ESCAPE '\\')
              ORDER BY p.name, p.version DESC LIMIT ?`,
          )
          .all(pattern, pattern, limit)
          .map((row) => ({ ...entry(row), readme: row.readme?.slice(0, 400) ?? null }));
      }
      const rows = principalId
        ? db
            .prepare(
              `SELECT * FROM packages p
                WHERE p.blocked = 0
                  AND (p.name LIKE ? ESCAPE '\\' OR p.readme LIKE ? ESCAPE '\\')
                  AND (p.visibility = 'public'
                       OR EXISTS (SELECT 1 FROM memberships m
                                   WHERE m.namespace = p.namespace AND m.principal_id = ?))
                ORDER BY p.name, p.version DESC LIMIT ?`,
            )
            .all(pattern, pattern, principalId, limit)
        : db
            .prepare(
              `SELECT * FROM packages p
                WHERE p.blocked = 0 AND p.visibility = 'public'
                  AND (p.name LIKE ? ESCAPE '\\' OR p.readme LIKE ? ESCAPE '\\')
                ORDER BY p.name, p.version DESC LIMIT ?`,
            )
            .all(pattern, pattern, limit);
      return rows.map((row) => ({ ...entry(row), readme: row.readme?.slice(0, 400) ?? null }));
    },

    statementsVisible(principalId, since = 0, limit = 200, all = false) {
      if (all) {
        return db
          .prepare(
            `SELECT s.*, p.namespace AS package_namespace, p.visibility AS package_visibility
               FROM statements s JOIN packages p
               ON p.name = s.name AND p.version = s.version
              WHERE s.seq > ?
              ORDER BY s.seq LIMIT ?`,
          )
          .all(since, limit);
      }
      if (!principalId) {
        return db
          .prepare(
            `SELECT s.*, p.namespace AS package_namespace, p.visibility AS package_visibility
               FROM statements s JOIN packages p
               ON p.name = s.name AND p.version = s.version
              WHERE s.seq > ? AND p.visibility = 'public'
              ORDER BY s.seq LIMIT ?`,
          )
          .all(since, limit);
      }
      return db
        .prepare(
          `SELECT s.*, p.namespace AS package_namespace, p.visibility AS package_visibility
             FROM statements s JOIN packages p
             ON p.name = s.name AND p.version = s.version
            WHERE s.seq > ?
              AND (p.visibility = 'public'
                   OR EXISTS (SELECT 1 FROM memberships m
                               WHERE m.namespace = p.namespace AND m.principal_id = ?))
            ORDER BY s.seq LIMIT ?`,
        )
        .all(since, principalId, limit);
    },

    allStatements(since = 0, limit = 200) {
      return db.prepare('SELECT * FROM statements WHERE seq > ? ORDER BY seq LIMIT ?').all(since, limit);
    },

    addStatement(kind, row, reason, issuerId, seq) {
      const statement = {
        _type: 'https://in-toto.io/Statement/v1',
        subject: [
          {
            name: `${row.name}@${row.version}`,
            digest: { sha256: row.package_id.replace(/^sha256:/, '') },
          },
        ],
        predicateType: 'https://cinch.systems/attestation/withdrawal/v1',
        predicate: {
          kind,
          scope: 'release',
          packageId: row.package_id,
          name: row.name,
          version: row.version,
          reason,
          issuer: issuerId,
          sequence: seq,
          timestamp: now(),
        },
      };
      const signed = signer.signStatement(statement);
      db.prepare(
        `INSERT INTO statements(seq, kind, name, version, reason, issuer, created_at, payload_json, envelope_json, digest)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
      ).run(
        seq,
        kind,
        row.name,
        row.version,
        reason,
        issuerId,
        now(),
        signed.payloadJson,
        canonicalJson(signed.envelope),
        signed.digest,
      );
      return { seq, ...signed };
    },

    transition(name, version, kind, reason, issuerId, mutate) {
      const row = store.getVersion(name, version);
      if (!row) throw notFound(`${name}@${version} does not exist`);
      const sequence = store.nextSequence();
      db.exec('BEGIN');
      try {
        mutate(row);
        db.prepare('UPDATE packages SET yanked = ?, blocked = ?, block_reason = ?, sequence = ? WHERE name = ? AND version = ?').run(
          row.yanked,
          row.blocked,
          row.block_reason,
          sequence,
          name,
          version,
        );
        const statement = store.addStatement(kind, row, reason, issuerId, sequence);
        db.exec('COMMIT');
        const updated = store.getVersion(name, version);
        return { entry: entry(updated), statement };
      } catch (error) {
        db.exec('ROLLBACK');
        throw error;
      }
    },

    yank(name, version, reason, issuerId) {
      return store.transition(name, version, 'yank', reason, issuerId, (row) => {
        row.yanked = 1;
      });
    },

    takedown(name, version, reason, issuerId) {
      return store.transition(name, version, 'takedown', reason, issuerId, (row) => {
        row.blocked = 1;
        row.block_reason = reason;
      });
    },

    restore(name, version, reason, issuerId) {
      return store.transition(name, version, 'restore', reason, issuerId, (row) => {
        row.blocked = 0;
        row.block_reason = null;
      });
    },

    packageForDownload(pid) {
      const row = store.getByPackageId(pid);
      if (!row) throw notFound('package does not exist');
      if (row.blocked) throw gone(`${row.name}@${row.version} is blocked`);
      return row;
    },
  };

  store.ensureAdmin();
  return store;
}
