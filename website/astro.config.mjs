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
      const link = relPath === 'overview.md' ? '/' : `/${relPath.replace(/\.md$/, '')}/`;
      return { label: title, link };
    });
}

export default defineConfig({
  // Placeholder for canonical URLs and the sitemap; set the real origin on deploy.
  site: 'https://safelang.invalid',
  trailingSlash: 'always',
  integrations: [
    starlight({
      title: 'Designing a Safe Language',
      description:
        'Draft language design documentation: a strict, dependently typed functional language with explicit effects, resource control, and checked deployment.',
      favicon: '/favicon.svg',
      head: [],
      customCss: ['./src/styles/custom.css'],
      components: {
        PageTitle: './src/components/PageTitle.astro',
      },
      lastUpdated: false,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
      sidebar: [
        { label: 'Overview', link: '/' },
        { label: 'Specifications', items: pagesIn('specifications') },
        { label: 'Tutorials', items: pagesIn('tutorials') },
        { label: 'Design', items: pagesIn('design') },
        { label: 'Examples', items: pagesIn('examples') },
        { label: 'Background', items: pagesIn('context') },
      ],
    }),
  ],
});
