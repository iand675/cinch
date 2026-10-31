// @ts-check
import { readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

const REPO_ROOT = fileURLToPath(new URL('..', import.meta.url));

/**
 * Navigation is derived from the Markdown itself, so a new chapter shows up
 * without editing this file. The page title is the leading `# heading`, the
 * same rule the content loader uses. Filename order is reading order: tutorial
 * filenames are zero-padded, so plain sorting gives 1 through 14.
 *
 * @param {string} dir
 * @returns {{ label: string, link: string }[]}
 */
function pagesIn(dir) {
  return readdirSync(path.join(REPO_ROOT, dir))
    .filter((name) => name.endsWith('.md'))
    .sort()
    .map((name) => {
      const relPath = dir ? `${dir}/${name}` : name;
      const source = readFileSync(path.join(REPO_ROOT, relPath), 'utf8');
      const title = /^#\s+(.+)$/m.exec(source)?.[1]?.trim() ?? path.basename(name, '.md');
      const link = `/${relPath.replace(/\.md$/, '')}/`;
      return { label: title, link };
    });
}

export default defineConfig({
  site: 'https://cinch.systems',
  trailingSlash: 'always',
  integrations: [
    starlight({
      title: 'Cinch',
      description:
        'Cinch is a strict functional language with dependent types, explicit effects, resource ownership, and checked contracts for running software.',
      favicon: '/favicon.svg',
      head: [
        { tag: 'meta', attrs: { name: 'theme-color', content: '#171715' } },
        { tag: 'link', attrs: { rel: 'preload', href: '/fonts/momo-trust-display.ttf', as: 'font', type: 'font/ttf', crossorigin: '' } },
      ],
      customCss: ['./src/styles/custom.css', './src/styles/docs-material.css'],
      components: {
        PageTitle: './src/components/PageTitle.astro',
        MarkdownContent: './src/components/MarkdownContent.astro',
        Header: './src/components/Header.astro',
        Sidebar: './src/components/Sidebar.astro',
      },
      lastUpdated: false,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
      sidebar: [
        { label: 'Language overview', link: '/overview/' },
        { label: '01 / Learn Cinch', items: pagesIn('tutorials') },
        { label: '02 / Language reference', items: pagesIn('specifications'), collapsed: true },
        { label: '03 / Design notes', collapsed: true, items: pagesIn('design') },
        { label: '04 / Examples', collapsed: true, items: pagesIn('examples') },
        { label: '05 / Background', collapsed: true, items: pagesIn('context') },
      ],
    }),
  ],
});
