import http from 'node:http';
import { randomBytes } from 'node:crypto';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { openStore } from './lib/store.mjs';
import { openSigner } from './lib/signer.mjs';
import {
  HttpError,
  badRequest,
  forbidden,
  notFound,
  tooLarge,
  unauthorized,
} from './lib/errors.mjs';

const routes = [];

function route(method, pattern, handler) {
  routes.push({ method, parts: pattern.split('/').filter(Boolean), handler });
}

function decodeSegment(value) {
  try {
    return decodeURIComponent(value);
  } catch {
    throw badRequest('invalid URL encoding');
  }
}

function matchRoute(pattern, parts) {
  if (pattern.length !== parts.length) return null;
  const params = {};
  for (let i = 0; i < pattern.length; i += 1) {
    const expected = pattern[i];
    if (expected.startsWith(':')) params[expected.slice(1)] = parts[i];
    else if (expected !== parts[i]) return null;
  }
  return params;
}

function authenticate(req, store) {
  const header = req.headers.authorization;
  if (!header) return null;
  const [scheme, token] = header.split(' ');
  if (scheme !== 'Bearer' || !token) return null;
  return store.findToken(token);
}

const isAdmin = (auth) => Boolean(auth?.scopes.includes('admin'));

function tokenAllowsNamespace(auth, namespace) {
  if (!auth) return false;
  return auth.namespaces.includes('*') || auth.namespaces.includes(namespace);
}

function requireAuth(auth) {
  if (!auth) throw unauthorized();
  return auth;
}

function requireAdmin(auth) {
  if (!isAdmin(auth)) throw forbidden('admin token required');
}

function canRead(auth, row) {
  if (row.visibility === 'public') return true;
  if (!auth) return false;
  if (isAdmin(auth)) return true;
  if (!auth.scopes.includes('read')) return false;
  if (!tokenAllowsNamespace(auth, row.namespace)) return false;
  return Boolean(auth.memberships[row.namespace]);
}

function canPublish(auth, namespace) {
  if (!auth) return false;
  if (isAdmin(auth)) return true;
  if (!auth.scopes.includes('publish')) return false;
  if (!tokenAllowsNamespace(auth, namespace)) return false;
  const role = auth.memberships[namespace];
  return role === 'owner' || role === 'publisher';
}

async function readJson(req, limit = 32 * 1024 * 1024) {
  const chunks = [];
  let size = 0;
  for await (const chunk of req) {
    size += chunk.length;
    if (size > limit) throw tooLarge('request body is too large');
    chunks.push(chunk);
  }
  if (size === 0) return {};
  const text = Buffer.concat(chunks).toString('utf8');
  try {
    return JSON.parse(text);
  } catch {
    throw badRequest('request body is not valid JSON');
  }
}

function sendJson(res, status, value, headers = {}) {
  const body = JSON.stringify(value, null, 2);
  res.writeHead(status, {
    'content-type': 'application/json; charset=utf-8',
    'content-length': Buffer.byteLength(body),
    'cache-control': 'no-store',
    'x-content-type-options': 'nosniff',
    ...headers,
  });
  res.end(body);
}

function sendHtml(res, status, body) {
  res.writeHead(status, {
    'content-type': 'text/html; charset=utf-8',
    'content-length': Buffer.byteLength(body),
    'cache-control': 'no-store',
    'x-content-type-options': 'nosniff',
  });
  res.end(body);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function statementRow(row) {
  return {
    seq: row.seq,
    kind: row.kind,
    name: row.name,
    version: row.version,
    reason: row.reason,
    issuer: row.issuer,
    createdAt: row.created_at,
    digest: row.digest,
    envelope: JSON.parse(row.envelope_json),
  };
}

function sinceOf(url) {
  const value = Number(url.searchParams.get('since') ?? 0);
  return Number.isFinite(value) && value > 0 ? Math.floor(value) : 0;
}

function reasonOf(value, fallback) {
  if (value === undefined || value === null) return fallback;
  if (typeof value !== 'string' || value.length === 0 || value.length > 500) {
    throw badRequest('reason must be a non-empty string of at most 500 characters');
  }
  return value;
}

function page(title, body) {
  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>${escapeHtml(title)} | Cinch registry</title>
<style>
:root { color-scheme: light dark; --fg:#18181b; --muted:#52525b; --bg:#fafafa; --card:#fff; --line:#e4e4e7; --accent:#0f766e; }
@media (prefers-color-scheme: dark) { :root { --fg:#f4f4f5; --muted:#a1a1aa; --bg:#09090b; --card:#18181b; --line:#27272a; --accent:#2dd4bf; } }
* { box-sizing: border-box; }
body { margin:0; font:15px/1.55 ui-sans-serif, system-ui, -apple-system, sans-serif; color:var(--fg); background:var(--bg); }
header { border-bottom:1px solid var(--line); padding:18px 24px; }
header a { color:var(--fg); text-decoration:none; font-weight:650; letter-spacing:-0.01em; }
main { max-width:920px; margin:0 auto; padding:32px 24px 64px; }
h1 { font-size:32px; letter-spacing:-0.03em; margin:0 0 8px; }
h2 { font-size:18px; margin:0; letter-spacing:-0.01em; }
p { color:var(--muted); }
code, pre { font-family:ui-monospace, SFMono-Regular, Menlo, monospace; }
code { background:var(--card); border:1px solid var(--line); border-radius:5px; padding:1px 5px; }
pre { background:var(--card); border:1px solid var(--line); border-radius:8px; padding:14px; overflow:auto; }
form { display:flex; gap:8px; margin:20px 0 28px; }
input { flex:1; padding:10px 12px; border:1px solid var(--line); border-radius:7px; background:var(--card); color:var(--fg); font:inherit; }
button { padding:10px 16px; border:1px solid var(--line); border-radius:7px; background:var(--accent); color:#fff; font:inherit; cursor:pointer; }
.grid { display:grid; gap:12px; }
.card { border:1px solid var(--line); background:var(--card); border-radius:10px; padding:16px 18px; }
.card a { color:var(--fg); text-decoration:none; }
.card a:hover { color:var(--accent); }
.meta { display:flex; gap:14px; flex-wrap:wrap; font-size:13px; color:var(--muted); margin-top:8px; }
table { width:100%; border-collapse:collapse; margin:18px 0; }
th, td { text-align:left; border-bottom:1px solid var(--line); padding:9px 8px; }
th { font-size:12px; text-transform:uppercase; letter-spacing:.06em; color:var(--muted); }
.badge { display:inline-block; font-size:11px; text-transform:uppercase; letter-spacing:.06em; border:1px solid var(--line); border-radius:999px; padding:2px 8px; color:var(--muted); }
footer { max-width:920px; margin:0 auto; padding:0 24px 40px; color:var(--muted); font-size:13px; }
</style>
</head>
<body>
<header><a href="/">Cinch registry</a></header>
<main>${body}</main>
<footer>Public package index. Private packages are available through the authenticated API.</footer>
</body>
</html>`;
}

function renderIndex(store, query) {
  const rows = store.search(query, null, 100);
  const byName = new Map();
  for (const row of rows) {
    if (!byName.has(row.name)) byName.set(row.name, []);
    byName.get(row.name).push(row);
  }
  const cards = [...byName.entries()]
    .map(([name, versions]) => {
      const latest = versions[0];
      const summary = latest.readme
        ? latest.readme.replace(/\s+/g, ' ').slice(0, 180)
        : 'No readme.';
      return `<article class="card">
        <h2><a href="/packages/${encodeURIComponent(name)}">${escapeHtml(name)}</a></h2>
        <div class="meta"><span>${escapeHtml(latest.version)}</span><span>${versions.length} version${versions.length === 1 ? '' : 's'}</span><span class="badge">public</span></div>
        <p>${escapeHtml(summary)}</p>
      </article>`;
    })
    .join('\n');
  return page(
    'Packages',
    `<h1>Packages</h1>
     <p>Public packages published to this registry.</p>
     <form action="/" method="get"><input name="q" value="${escapeHtml(query)}" placeholder="Search packages" aria-label="Search packages"><button type="submit">Search</button></form>
     <div class="grid">${cards || '<p>No packages yet.</p>'}</div>`,
  );
}

function renderPackage(store, name) {
  const rows = store
    .getVersions(name)
    .filter((row) => row.visibility === 'public' && !row.blocked);
  if (rows.length === 0) return null;
  const readme = rows.find((row) => row.readme)?.readme ?? '';
  const manifest = JSON.parse(rows[0].manifest_json);
  const versions = rows
    .map((row) => {
      const hex = row.package_id.replace(/^sha256:/, '');
      return `<tr>
        <td><code>${escapeHtml(row.version)}</code></td>
        <td>${row.yanked ? '<span class="badge">yanked</span>' : '<span class="badge">active</span>'}</td>
        <td><a href="/api/v1/download/${hex}">download</a></td>
      </tr>`;
    })
    .join('');
  return page(
    name,
    `<h1>${escapeHtml(name)}</h1>
     <p>Public package with ${manifest.modules.length} module${manifest.modules.length === 1 ? '' : 's'}.</p>
     <table><thead><tr><th>Version</th><th>Status</th><th>Artifact</th></tr></thead><tbody>${versions}</tbody></table>
     <h2>Install</h2>
     <pre><code>cinch pkg add ${escapeHtml(name)}@${escapeHtml(rows[0].version)}</code></pre>
     <h2>Readme</h2>
     <pre>${escapeHtml(readme || 'No readme.')}</pre>`,
  );
}

function handleRequest(req, res, store, signer) {
  const url = new URL(req.url, 'http://registry.local');
  if (req.method === 'OPTIONS') {
    res.writeHead(204, {
      'access-control-allow-origin': '*',
      'access-control-allow-headers': 'authorization, content-type',
      'access-control-allow-methods': 'GET, POST, DELETE, OPTIONS',
    });
    res.end();
    return;
  }
  res.setHeader('access-control-allow-origin', '*');
  const parts = url.pathname.split('/').filter(Boolean).map(decodeSegment);
  for (const candidate of routes) {
    if (candidate.method !== req.method) continue;
    const params = matchRoute(candidate.parts, parts);
    if (params) return candidate.handler({ req, res, url, params, store, signer });
  }
  throw notFound('no such route');
}

// ---------------------------------------------------------------- API

route('GET', '/health', ({ res }) => {
  sendJson(res, 200, { ok: true, service: 'cinch-registry', version: '0.1.0' });
});

route('GET', '/api/v1', ({ res }) => {
  sendJson(res, 200, {
    service: 'cinch-registry',
    version: '0.1.0',
    endpoints: [
      'POST /api/v1/principals',
      'POST /api/v1/namespaces',
      'POST /api/v1/namespaces/:namespace/members',
      'POST /api/v1/tokens',
      'POST /api/v1/packages',
      'GET /api/v1/index',
      'GET /api/v1/index/private',
      'GET /api/v1/search?q=',
      'GET /api/v1/packages/:name',
      'GET /api/v1/packages/:name/:version',
      'GET /api/v1/download/:packageId',
      'POST /api/v1/packages/:name/:version/yank',
      'POST /api/v1/packages/:name/:version/takedown',
      'POST /api/v1/packages/:name/:version/restore',
      'GET /api/v1/statements?since=',
      'GET /api/v1/registry-key',
    ],
  });
});

route('POST', '/api/v1/principals', async ({ req, res, store }) => {
  const body = await readJson(req);
  const created = store.createUser(body.handle);
  sendJson(res, 201, {
    principal: created.principal,
    namespace: created.principal.handle,
    token: created.token,
  });
});

route('POST', '/api/v1/namespaces', async ({ req, res, store, url }) => {
  const auth = requireAuth(authenticate(req, store));
  const body = await readJson(req);
  if (body.kind !== 'org') throw badRequest('kind must be "org"');
  const created = store.createOrg(body.name, auth.principal.id);
  sendJson(res, 201, created);
});

route('POST', '/api/v1/namespaces/:namespace/members', async ({ req, res, store, params }) => {
  const auth = requireAuth(authenticate(req, store));
  const namespace = params.namespace;
  if (!isAdmin(auth) && auth.memberships[namespace] !== 'owner') {
    throw forbidden(`only an owner of ${namespace} can manage members`);
  }
  const body = await readJson(req);
  if (typeof body.handle !== 'string') throw badRequest('handle is required');
  const principal = store.principalByHandle(body.handle);
  if (!principal) throw notFound(`principal "${body.handle}" does not exist`);
  const member = store.addMember(namespace, principal.id, body.role);
  sendJson(res, 200, member);
});

route('POST', '/api/v1/tokens', async ({ req, res, store }) => {
  const auth = requireAuth(authenticate(req, store));
  const body = await readJson(req);
  const scopes = body.scopes;
  const namespaces = body.namespaces;
  if (!Array.isArray(scopes)) throw badRequest('scopes must be an array');
  if (!Array.isArray(namespaces)) throw badRequest('namespaces must be an array');
  const targetHandle = body.principal ?? auth.principal.handle;
  if (typeof targetHandle !== 'string') throw badRequest('principal must be a handle');
  const target = store.principalByHandle(targetHandle);
  if (!target) throw notFound(`principal "${targetHandle}" does not exist`);
  if (target.id !== auth.principal.id && !isAdmin(auth)) {
    throw forbidden('only an admin can mint a token for another principal');
  }
  if (scopes.includes('admin') && !isAdmin(auth)) {
    throw forbidden('only an admin can mint an admin token');
  }
  if (namespaces.includes('*') && !isAdmin(auth)) {
    throw forbidden('only an admin can mint a wildcard token');
  }
  const memberships = store.membershipsFor(target.id);
  for (const namespace of namespaces) {
    if (namespace === '*') continue;
    if (!isAdmin(auth) && !memberships[namespace]) {
      throw forbidden(`principal "${targetHandle}" is not a member of "${namespace}"`);
    }
  }
  const created = store.createToken(target.id, scopes, namespaces, body.expiresAt ?? null);
  sendJson(res, 201, created);
});

route('DELETE', '/api/v1/tokens/:id', ({ req, res, store, params }) => {
  const auth = requireAuth(authenticate(req, store));
  store.revokeToken(params.id, auth.principal.id, isAdmin(auth));
  res.writeHead(204);
  res.end();
});

route('POST', '/api/v1/packages', async ({ req, res, store }) => {
  const auth = requireAuth(authenticate(req, store));
  const body = await readJson(req);
  const manifest = body.manifest;
  const namespace = typeof manifest?.name === 'string' ? manifest.name.split('/')[0] : null;
  if (!namespace) throw badRequest('manifest.name must be namespace/name');
  if (!canPublish(auth, namespace)) {
    throw forbidden(`no publish permission for ${namespace}`);
  }
  const visibility =
    body.visibility ?? (store.namespace(namespace)?.kind === 'org' ? 'private' : 'public');
  const published = await store.publish({
    manifest,
    files: body.files,
    visibility,
    readme: body.readme,
    attestations: body.attestations,
    publisherId: auth.principal.id,
    claimedPackageId: body.packageId,
  });
  sendJson(res, 201, published);
});

route('GET', '/api/v1/packages/:name', ({ req, res, store, params }) => {
  const auth = authenticate(req, store);
  const rows = store.getVersions(params.name).filter((row) => canRead(auth, row));
  if (rows.length === 0) throw notFound(`${params.name} does not exist`);
  sendJson(res, 200, {
    name: params.name,
    versions: rows.map((row) => ({
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
    })),
  });
});

route('GET', '/api/v1/packages/:name/:version', ({ req, res, store, params }) => {
  const auth = authenticate(req, store);
  const row = store.getVersion(params.name, params.version);
  if (!row || !canRead(auth, row)) throw notFound('package does not exist');
  sendJson(res, 200, {
    name: row.name,
    version: row.version,
    packageId: row.package_id,
    bundle: row.bundle_digest,
    visibility: row.visibility,
    namespace: row.namespace,
    publisher: row.publisher_id,
    yanked: Boolean(row.yanked),
    blocked: Boolean(row.blocked),
    manifest: JSON.parse(row.manifest_json),
    files: JSON.parse(row.files_json),
    readme: row.readme,
    attestations: store.attestations(row.package_id),
    sequence: row.sequence,
    createdAt: row.created_at,
  });
});

route('GET', '/api/v1/download/:id', async ({ req, res, store, params }) => {
  const auth = authenticate(req, store);
  if (!/^[0-9a-f]{64}$/.test(params.id)) throw badRequest('package id must be a SHA-256 hex digest');
  const row = store.packageForDownload(`sha256:${params.id}`);
  if (!canRead(auth, row)) throw notFound('package does not exist');
  const bytes = await store.readBlob(row.bundle_digest);
  res.writeHead(200, {
    'content-type': 'application/vnd.cinch.package+json',
    'content-length': bytes.length,
    'content-disposition': `attachment; filename="${row.name.replace('/', '-')}-${row.version}.cinchpkg.json"`,
    'x-cinch-package-id': row.package_id,
    'x-cinch-yanked': String(Boolean(row.yanked)),
    'cache-control': 'no-store',
    'x-content-type-options': 'nosniff',
  });
  res.end(bytes);
});

route('GET', '/api/v1/index', ({ res, url, store }) => {
  const since = sinceOf(url);
  const packages = store.publicIndex(since);
  sendJson(
    res,
    200,
    { schema: 1, sequence: store.indexSequence(), packages },
    { 'cache-control': 'public, max-age=30' },
  );
});

route('GET', '/api/v1/index/private', ({ req, res, url, store }) => {
  const auth = requireAuth(authenticate(req, store));
  const since = sinceOf(url);
  const packages = store
    .privateIndex(auth.principal.id, since, isAdmin(auth))
    .filter(
      (row) =>
        row.visibility === 'public' || isAdmin(auth) || tokenAllowsNamespace(auth, row.namespace),
    );
  sendJson(res, 200, {
    schema: 1,
    sequence: store.indexSequence(),
    packages,
  });
});

route('GET', '/api/v1/search', ({ req, res, url, store }) => {
  const auth = authenticate(req, store);
  const query = (url.searchParams.get('q') ?? '').trim();
  const results = store
    .search(query, auth?.principal.id ?? null, 50, isAdmin(auth))
    .filter(
      (row) =>
        row.visibility === 'public' || isAdmin(auth) || tokenAllowsNamespace(auth, row.namespace),
    );
  sendJson(res, 200, { query, results });
});

route('GET', '/api/v1/statements', ({ req, res, url, store }) => {
  const auth = authenticate(req, store);
  const since = sinceOf(url);
  const statements = store
    .statementsVisible(auth?.principal.id ?? null, since, 200, isAdmin(auth))
    .filter(
      (row) =>
        row.package_visibility === 'public' ||
        isAdmin(auth) ||
        tokenAllowsNamespace(auth, row.package_namespace),
    )
    .map(statementRow);
  sendJson(res, 200, { sequence: store.indexSequence(), statements });
});

route('GET', '/api/v1/audit', ({ req, res, url, store }) => {
  const auth = requireAuth(authenticate(req, store));
  requireAdmin(auth);
  const since = sinceOf(url);
  sendJson(res, 200, {
    sequence: store.indexSequence(),
    statements: store.allStatements(since).map(statementRow),
  });
});

route('GET', '/api/v1/registry-key', ({ res, signer }) => {
  sendJson(res, 200, signer.publicInfo());
});

route('POST', '/api/v1/packages/:name/:version/yank', async ({ req, res, store, params }) => {
  const auth = requireAuth(authenticate(req, store));
  const row = store.getVersion(params.name, params.version);
  if (!row) throw notFound('package does not exist');
  if (!canPublish(auth, row.namespace)) throw forbidden('no publish permission');
  const body = await readJson(req);
  const reason = reasonOf(body.reason, 'publisher yank');
  sendJson(res, 200, store.yank(params.name, params.version, reason, auth.principal.handle));
});

route('POST', '/api/v1/packages/:name/:version/takedown', async ({ req, res, store, params }) => {
  const auth = requireAuth(authenticate(req, store));
  requireAdmin(auth);
  const body = await readJson(req);
  const reason = reasonOf(body.reason, 'policy takedown');
  sendJson(res, 200, store.takedown(params.name, params.version, reason, auth.principal.handle));
});

route('POST', '/api/v1/packages/:name/:version/restore', async ({ req, res, store, params }) => {
  const auth = requireAuth(authenticate(req, store));
  requireAdmin(auth);
  const body = await readJson(req);
  const reason = reasonOf(body.reason, 'restore');
  sendJson(res, 200, store.restore(params.name, params.version, reason, auth.principal.handle));
});

// ---------------------------------------------------------------- HTML

route('GET', '/', ({ res, url, store }) => {
  sendHtml(res, 200, renderIndex(store, (url.searchParams.get('q') ?? '').trim()));
});

route('GET', '/packages/:name', ({ res, store, params }) => {
  const html = renderPackage(store, params.name);
  if (!html) throw notFound('package does not exist');
  sendHtml(res, 200, html);
});

// ---------------------------------------------------------------- server

export async function createRegistry({ dataDir, adminToken, provenanceKeys = {}, port = 8787, host = '127.0.0.1' }) {
  if (!adminToken) throw new Error('adminToken is required');
  const signer = await openSigner(dataDir);
  const store = await openStore(dataDir, { adminToken, signer, provenanceKeys });
  const server = http.createServer((req, res) => {
    Promise.resolve()
      .then(() => handleRequest(req, res, store, signer))
      .catch((error) => {
        if (res.headersSent) {
          res.destroy(error);
          return;
        }
        if (error instanceof HttpError) {
          sendJson(res, error.status, {
            error: { code: error.code, message: error.message, details: error.details },
          });
          return;
        }
        console.error(error);
        sendJson(res, 500, { error: { code: 'internal', message: 'internal error' } });
      });
  });
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(port, host, resolve);
  });
  const address = server.address();
  return {
    server,
    store,
    signer,
    url: `http://${host}:${address.port}`,
    close: () => new Promise((resolve) => server.close(resolve)),
  };
}

const isMain = process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href;
if (isMain) {
  const root = path.dirname(fileURLToPath(import.meta.url));
  const dataDir = process.env.CINCH_REGISTRY_DATA ?? path.join(root, 'data');
  const generated = !process.env.CINCH_REGISTRY_ADMIN_TOKEN;
  const adminToken = process.env.CINCH_REGISTRY_ADMIN_TOKEN ?? randomBytes(24).toString('hex');
  const port = Number(process.env.PORT ?? 8787);
  let provenanceKeys = {};
  if (process.env.CINCH_PROVENANCE_KEYS) {
    provenanceKeys = JSON.parse(process.env.CINCH_PROVENANCE_KEYS);
  }
  const registry = await createRegistry({ dataDir, adminToken, provenanceKeys, port });
  console.log(`Cinch registry listening on ${registry.url}`);
  console.log(`Data directory: ${dataDir}`);
  console.log(`Registry key: ${registry.signer.keyid}`);
  if (generated) console.log(`Admin token (store it now): ${adminToken}`);
}
