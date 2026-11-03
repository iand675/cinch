import type { Loader } from 'astro/loaders';
import { readdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const SITE_ROOT = fileURLToPath(new URL('../..', import.meta.url));
const REPO_ROOT = path.dirname(SITE_ROOT);

/**
 * Directories of the documentation set, relative to the repository root.
 * Every Markdown file in them becomes a page and the route is its path without
 * the extension; the overview is served at /overview/.
 */
const DOC_DIRS = ['', 'learn', 'reference', 'library', 'specifications', 'tutorials', 'design', 'examples', 'context'];

type PageSource = {
  /** Path used to resolve links between pages, relative to the repository root. */
  relPath: string;
  /** Absolute path of the Markdown file. */
  file: string;
  /** Route id: the path without its extension, including `overview` for the design overview. */
  id: string;
};

type PageMeta = {
  title: string;
  source?: string;
  exported?: string;
  added?: string;
};

const META_LINE = /^(Source|Exported|Added):\s*(.+?)\s*$/;

/**
 * Fence languages the highlighter does not carry. `ebnf` is not a Shiki
 * grammar, and the block is plain notation, so it renders as text.
 */
const FENCE_LANGUAGE_ALIASES: Record<string, string> = { ebnf: 'text' };

/** Strip the site-absolute prefix of a link written for the raw Markdown files. */
function routeFor(relPath: string): string {
  return `/${relPath.replace(/\.md$/, '')}/`;
}

async function markdownFiles(root: string, relDir: string): Promise<string[]> {
  const entries = await readdir(path.join(root, relDir), { withFileTypes: true });
  return entries
    .filter((entry) => entry.isFile() && entry.name.endsWith('.md'))
    .map((entry) => entry.name)
    .sort();
}

async function collectSources(): Promise<PageSource[]> {
  const sources: PageSource[] = [];

  for (const dir of DOC_DIRS) {
    for (const name of await markdownFiles(REPO_ROOT, dir)) {
      // The repository root also holds README.md; only the overview is a page.
      if (dir === '' && !['overview.md', 'documentation.md'].includes(name)) continue;
      const relPath = dir === '' ? name : `${dir}/${name}`;
      sources.push({
        relPath,
        file: path.join(REPO_ROOT, relPath),
        id: relPath.replace(/\.md$/, ''),
      });
    }
  }

  return sources;
}

/**
 * Split the export header (title, provenance, rule) from the document body and
 * turn the provenance lines into page data, so the site renders them as chrome
 * instead of body text.
 */
function parsePage(raw: string, relPath: string): { meta: PageMeta; body: string } {
  const lines = raw.split('\n');
  const meta: PageMeta = { title: path.basename(relPath, '.md') };

  let index = 0;
  while (index < lines.length && lines[index]!.trim() === '') index += 1;

  if (lines[index]?.startsWith('# ')) {
    meta.title = lines[index]!.slice(2).trim();
    index += 1;
  }

  let sawMeta = false;
  while (index < lines.length) {
    const line = lines[index]!.trim();
    if (line === '') {
      index += 1;
      continue;
    }
    if (sawMeta && line === '---') {
      index += 1;
      break;
    }
    const match = META_LINE.exec(line);
    if (!match) break;
    const [, key, value] = match as unknown as [string, string, string];
    const clean = value.replace(/^<(.*)>$/, '$1').trim();
    if (key === 'Source') meta.source = clean;
    else if (key === 'Exported') meta.exported = clean;
    else if (key === 'Added') meta.added = clean;
    sawMeta = true;
    index += 1;
  }

  while (index < lines.length && lines[index]!.trim() === '') index += 1;

  return { meta, body: lines.slice(index).join('\n') };
}

/**
 * Prepare a page body for the site.
 *
 * Sibling Markdown links become site routes; links that already point at a URL,
 * an anchor, or anything that is not a documentation page are left untouched.
 * Every heading is demoted one level, because the documents use `#` for
 * numbered sections while the site renders the page title as the only `h1`.
 * Fenced code blocks are skipped so their contents never change.
 */
function transformBody(
  body: string,
  relPath: string,
  knownPages: Set<string>,
  warn: (message: string) => void,
): string {
  const dir = path.posix.dirname(relPath);
  const out: string[] = [];
  let fence: string | null = null;

  for (const line of body.split('\n')) {
    const fenceMatch = /^(\s*)(```+|~~~+)([\w+-]*)(.*)$/.exec(line);
    if (fenceMatch) {
      const [, indent, run, language, rest] = fenceMatch as unknown as [
        string,
        string,
        string,
        string,
        string,
      ];
      const marker = run[0]!;
      const opens = fence === null;
      if (opens) fence = marker;
      else if (fence === marker) fence = null;
      const alias = opens && language ? FENCE_LANGUAGE_ALIASES[language] : undefined;
      out.push(alias ? `${indent}${run}${alias}${rest}` : line);
      continue;
    }
    if (fence !== null) {
      out.push(line);
      continue;
    }

    const demoted = line.replace(/^(#{1,5})(\s+)/, '#$1$2');
    out.push(
      demoted.replace(
        /(\]\()([^)\s]+)((?:\s+"[^"]*")?\))/g,
        (whole, open: string, target: string, close: string) => {
          if (
            /^[a-z][a-z0-9+.-]*:/i.test(target) ||
            target.startsWith('//') ||
            target.startsWith('#')
          ) {
            return whole;
          }
          const [pathPart, ...rest] = target.split('#');
          const hash = rest.length > 0 ? `#${rest.join('#')}` : '';
          if (!pathPart) return whole;

          const resolved = path.posix.normalize(path.posix.join(dir, pathPart));
          const asMarkdown = resolved.endsWith('.md') ? resolved : `${resolved}.md`;
          if (knownPages.has(asMarkdown)) {
            return `${open}${routeFor(asMarkdown)}${hash}${close}`;
          }
          if (asMarkdown.startsWith('..')) {
            warn(`link target outside the documentation set in ${relPath}: ${target}`);
          }
          return whole;
        },
      ),
    );
  }

  return out.join('\n');
}

export function safelangDocs(): Loader {
  return {
    name: 'safelang-docs',
    async load({ store, logger, watcher, parseData, renderMarkdown, generateDigest }) {
      const sources = await collectSources();
      const knownPages = new Set(sources.map((source) => source.relPath));
      // Rendered HTML contains Expressive Code's token colours and hashed asset
      // URLs. A content-only digest would retain stale HTML after a theme/config
      // change, even though the matching stylesheet is no longer emitted.
      const renderingFiles = [
        path.join(SITE_ROOT, 'astro.config.mjs'),
        path.join(SITE_ROOT, 'src/themes/syntax.mjs'),
        path.join(REPO_ROOT, 'tooling/editors/vscode/syntaxes/cinch.tmLanguage.json'),
      ];
      const renderDigest = generateDigest((await Promise.all(renderingFiles.map(file => readFile(file, 'utf8')))).join('\n'));

      for (const { relPath, file, id } of sources) {
        const raw = await readFile(file, 'utf8');
        const { meta, body } = parsePage(raw, relPath);
        const rewritten = transformBody(body, relPath, knownPages, (message) =>
          logger.warn(message),
        );

        // parseData applies the collection schema, including the defaults
        // Starlight relies on (head, sidebar, draft, and so on).
        const data = await parseData({ id, data: { ...meta } });
        // A custom loader renders its own Markdown; Astro does not do it for us.
        const rendered = await renderMarkdown(rewritten);

        store.set({
          id,
          data,
          body: rewritten,
          rendered,
          // The digest covers the data as well as the body: the content layer
          // skips entries whose digest is unchanged.
          digest: generateDigest(JSON.stringify({ data, body: rewritten, renderDigest })),
        });

        watcher?.add(file);
      }

      logger.info(`loaded ${sources.length} documentation pages`);
    },
  };
}
