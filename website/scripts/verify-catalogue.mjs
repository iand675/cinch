#!/usr/bin/env node
/**
 * Build the static catalogue against a real local registry release.
 *
 * Starts registry/server.mjs on a free port with a temporary data directory,
 * creates a principal, publishes one public package, builds the site with
 * CINCH_CATALOGUE_REGISTRY pointed at that registry, and checks the generated
 * package, module, and source pages. Then runs verify-preview.py.
 *
 * Run from website/: node scripts/verify-catalogue.mjs
 */
import { spawn, spawnSync } from 'node:child_process';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = fileURLToPath(new URL('.', import.meta.url));
const site = path.resolve(here, '..');
const repo = path.resolve(site, '..');
const registryServer = path.join(repo, 'registry', 'server.mjs');
const data = await mkdtemp(path.join(os.tmpdir(), 'cinch-catalogue-'));
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function freePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.unref();
    server.on('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const port = server.address().port;
      server.close(() => resolve(port));
    });
  });
}

async function waitForHealth(base) {
  const deadline = Date.now() + 15000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`${base}/health`);
      if (response.ok) return;
    } catch {
      // The process is still starting.
    }
    await sleep(100);
  }
  throw new Error(`registry at ${base} did not become healthy`);
}

const port = await freePort();
const base = `http://127.0.0.1:${port}`;
const registry = spawn(process.execPath, [registryServer], {
  cwd: path.join(repo, 'registry'),
  env: {
    ...process.env,
    PORT: String(port),
    CINCH_REGISTRY_DATA: data,
    CINCH_REGISTRY_ADMIN_TOKEN: 'fixture-admin-token',
  },
  stdio: 'ignore',
});

async function stopRegistry() {
  if (registry.exitCode !== null) return;
  registry.kill('SIGTERM');
  await Promise.race([
    new Promise((resolve) => registry.once('exit', resolve)),
    sleep(3000),
  ]);
  if (registry.exitCode === null) registry.kill('SIGKILL');
}

try {
  await waitForHealth(base);

  const principalResponse = await fetch(`${base}/api/v1/principals`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ handle: 'fixture' }),
  });
  assert(principalResponse.status === 201, `principal creation returned ${principalResponse.status}`);
  const principal = await principalResponse.json();

  const source = Buffer.from('module Fixture.Hello where\n\n-- Published by the catalogue verification.\n');
  const publishResponse = await fetch(`${base}/api/v1/packages`, {
    method: 'POST',
    headers: {
      authorization: `Bearer ${principal.token}`,
      'content-type': 'application/json',
    },
    body: JSON.stringify({
      manifest: {
        schema: 2,
        name: 'fixture/hello',
        version: '1.2.3',
        license: 'BSD-3-Clause',
        description: 'A fixture release published by the catalogue verification.',
        modules: [{ name: 'Fixture.Hello', path: 'src/Fixture/Hello.cinch' }],
        order: ['Fixture.Hello'],
        dependencies: [{ name: 'cinch/base', range: '^0.1.0', source: 'registry' }],
      },
      files: { 'src/Fixture/Hello.cinch': source.toString('base64') },
      visibility: 'public',
      readme: '# Fixture Hello\n\nA package published by the website verification script.\n\nIt exists so the static catalogue can be built from a real registry release.',
    }),
  });
  const publishBody = await publishResponse.text();
  assert(publishResponse.status === 201, `publish returned ${publishResponse.status}: ${publishBody}`);
  const published = JSON.parse(publishBody);

  await rm(path.join(site, 'dist'), { recursive: true, force: true });
  const build = spawnSync('npm', ['run', 'build'], {
    cwd: site,
    env: { ...process.env, CINCH_CATALOGUE_REGISTRY: base, CINCH_CATALOGUE_OFFLINE: '' },
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  });
  if (build.status !== 0) {
    process.stderr.write(build.stdout ?? '');
    process.stderr.write(build.stderr ?? '');
    throw new Error(`npm run build failed with status ${build.status}`);
  }

  const catalogue = JSON.parse(await readFile(path.join(site, 'dist', 'packages', 'index.json'), 'utf8'));
  assert(catalogue.sample === false, 'catalogue reports sample data');
  assert(catalogue.offline === false, 'catalogue reports an offline build');
  assert(catalogue.registry === base, `catalogue registry is ${catalogue.registry}`);
  const pkg = catalogue.packages.find((item) => item.name === 'fixture/hello');
  assert(pkg, 'published package is missing from the catalogue');
  assert(pkg.version === '1.2.3', `latest version is ${pkg.version}`);
  assert(pkg.packageId === published.entry.packageId, 'catalogue identity does not match the published release');

  const packagePath = path.join(site, 'dist', 'packages', 'fixture', 'hello', '1.2.3');
  const packageHtml = await readFile(path.join(packagePath, 'index.html'), 'utf8');
  assert(packageHtml.includes('fixture/hello'), 'package page does not name the package');
  assert(packageHtml.includes('1.2.3'), 'package page does not name the version');
  assert(packageHtml.includes(base), 'package page does not name the registry');
  assert(packageHtml.includes(`--registry ${base}`), 'install command does not name the registry');
  assert(packageHtml.includes(published.entry.packageId), 'package page does not name the release identity');
  assert(!packageHtml.includes('SAMPLE DATA'), 'package page still says SAMPLE DATA');

  const modulePath = path.join(packagePath, 'Fixture.Hello');
  const moduleHtml = await readFile(path.join(modulePath, 'index.html'), 'utf8');
  assert(moduleHtml.includes('Fixture.Hello'), 'module page does not name the module');
  assert(moduleHtml.includes('src/Fixture/Hello.cinch'), 'module page does not name the manifest path');
  assert(moduleHtml.includes('.cinchdoc'), 'module page does not explain generated declaration data');
  assert(!moduleHtml.includes('SAMPLE API'), 'module page still says SAMPLE API');

  const sourceHtml = await readFile(path.join(modulePath, 'source', 'index.html'), 'utf8');
  assert(sourceHtml.includes('module Fixture.Hello where'), 'source page does not show the published source');
  assert(!sourceHtml.includes('SAMPLE LISTING'), 'source page still says SAMPLE LISTING');

  const preview = spawnSync('python3', ['scripts/verify-preview.py'], { cwd: site, encoding: 'utf8' });
  if (preview.status !== 0) {
    process.stderr.write(preview.stdout ?? '');
    process.stderr.write(preview.stderr ?? '');
    throw new Error(`verify-preview.py failed with status ${preview.status}`);
  }

  console.log(`Catalogue fixture verified: fixture/hello@1.2.3 from ${base}`);
  console.log(preview.stdout.trim());
} finally {
  await stopRegistry();
  await rm(data, { recursive: true, force: true });
}
