// @ts-check
import { readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { syntaxThemes } from './src/themes/syntax.mjs';

const REPO_ROOT = fileURLToPath(new URL('..', import.meta.url));
const cinchGrammar = JSON.parse(readFileSync(path.join(REPO_ROOT, 'tooling/editors/vscode/syntaxes/cinch.tmLanguage.json'), 'utf8'));

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
      expressiveCode: {
        themes: syntaxThemes,
        useStarlightUiThemeColors: false,
        useStarlightDarkModeSwitch: true,
        shiki: { langs: [{ ...cinchGrammar, name: 'cinch', aliases: ['Cinch'] }] },
      },
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
      sidebar: [
        { label: 'Documentation home', link: '/documentation/' },
        { label: '01 / Language tutorial', items: pagesIn('learn') },
        { label: '02 / Compiler user’s guide', items: pagesIn('reference'), collapsed: true },
        { label: '03 / Built-in and shipped libraries', items: [
          { label: 'Library directory', link: '/library/00-index/' },
          ...['Prelude', 'Algebra', 'Category', 'Relation', 'Function', 'Data', 'Text', 'Effect'].map((family) => ({
            label: family,
            collapsed: true,
            items: pagesIn('library').filter((page) => page.label.split('.')[0] === family),
          })),
        ], collapsed: true },
        { label: '04 / Design archive', collapsed: true, items: [
          { label: 'Original design overview', link: '/overview/' },
          { label: 'Specifications', items: pagesIn('specifications'), collapsed: true },
          { label: 'Original design tutorials', items: pagesIn('tutorials'), collapsed: true },
          { label: 'Design notes', items: pagesIn('design'), collapsed: true },
          { label: 'Examples', items: pagesIn('examples'), collapsed: true },
          { label: 'Background', items: pagesIn('context'), collapsed: true },
        ] },
      ],
    }),
  ],
});
