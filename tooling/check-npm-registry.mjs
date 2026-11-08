import { execFileSync } from 'node:child_process';

const registry = 'https://registry.npmjs.org/';

function staged(path) {
  try {
    return execFileSync('git', ['show', `:${path}`], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
    });
  } catch {
    throw new Error(`Stage ${path} before committing.`);
  }
}

try {
  const settings = staged('website/.npmrc').split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line && !line.startsWith('#') && !line.startsWith(';'))
    .map((line) => line.split('=').map((part) => part.trim()));
  const registries = settings.filter(([name]) => name === 'registry' || name.endsWith(':registry'));
  if (registries.filter(([name]) => name === 'registry').length !== 1 ||
      registries.some(([, value, extra]) => value !== registry || extra !== undefined)) {
    throw new Error(`Set registry=${registry} in website/.npmrc and remove registry overrides.`);
  }

  const lock = JSON.parse(staged('website/package-lock.json'));
  if (lock.lockfileVersion !== 3 || typeof lock.packages !== 'object' ||
      !lock.packages || Array.isArray(lock.packages)) {
    throw new Error('Expected a version 3 npm lockfile for the website.');
  }
  for (const [name, pkg] of Object.entries(lock.packages)) {
    if (!pkg.link && pkg.resolved !== undefined &&
        (typeof pkg.resolved !== 'string' || !pkg.resolved.startsWith(registry))) {
      throw new Error(`Restore ${registry} tarball URLs in website/package-lock.json (first affected entry ${name}).`);
    }
  }
} catch (error) {
  console.error(`Website npm registry check failed. ${error.message}`);
  process.exitCode = 1;
}
