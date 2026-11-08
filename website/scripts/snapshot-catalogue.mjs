#!/usr/bin/env node
/**
 * Save the last successful live catalogue as the build fallback.
 *
 * Run a live build first:
 *   CINCH_CATALOGUE_REGISTRY=https://registry.example npm run build
 *   npm run catalogue:snapshot
 *
 * The file is committed with the site so a Vercel build can succeed when the
 * registry is down. Refresh it on whatever cadence the registry expects.
 */
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const site = path.resolve(here, '..');
const built = path.join(site, 'dist', 'packages', 'index.json');
const target = path.join(site, 'src', 'data', 'catalogue-snapshot.json');

const catalogue = JSON.parse(await readFile(built, 'utf8'));
if (catalogue.offline || catalogue.fallback) {
  throw new Error('the built catalogue is offline or a fallback; run a live build first');
}
if (!Array.isArray(catalogue.packages) || catalogue.packages.length === 0) {
  throw new Error('the built catalogue has no public releases');
}
await writeFile(target, `${JSON.stringify(catalogue, null, 2)}\n`);
console.log(`Wrote ${catalogue.packages.length} packages from ${catalogue.registry} to ${path.relative(site, target)}`);
