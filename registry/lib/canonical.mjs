import { createHash, randomBytes } from 'node:crypto';

/** Canonical JSON: sorted object keys, no whitespace, arrays preserved. */
export function canonicalJson(value) {
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(',')}]`;
  const keys = Object.keys(value).sort();
  return `{${keys.map((k) => `${JSON.stringify(k)}:${canonicalJson(value[k])}`).join(',')}}`;
}

export function sha256Hex(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

export function newToken() {
  return randomBytes(32).toString('hex');
}

export function tokenHash(token) {
  return sha256Hex(Buffer.from(token, 'utf8'));
}

export function randomId(prefix) {
  return `${prefix}_${randomBytes(12).toString('hex')}`;
}

/**
 * The release identity from design/package-management.md section 2:
 * a domain-separated SHA-256 over the canonical manifest and every source
 * file's path and byte digest, with the manifest excluded from the file list.
 */
export function packageId(manifest, files) {
  const entries = Object.keys(files)
    .sort()
    .map((path) => `${path}\t${sha256Hex(files[path])}`);
  const text = `package\n${canonicalJson(manifest)}\n${entries.join('\n')}`;
  return `sha256:${sha256Hex(Buffer.from(text, 'utf8'))}`;
}

export function canonicalDigest(value) {
  return `sha256:${sha256Hex(Buffer.from(canonicalJson(value), 'utf8'))}`;
}
