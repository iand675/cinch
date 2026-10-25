// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

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
        {
          label: 'Specifications',
          items: [
            { label: 'Core language', link: '/specifications/core-language-specification/' },
            { label: 'Primitive types', link: '/specifications/primitive-types-specification/' },
            { label: 'Surface syntax and grammar', link: '/specifications/surface-syntax-and-grammar/' },
            { label: 'Surface grammar local amendments', link: '/specifications/surface-grammar-local-amendments/' },
            { label: 'Modules and compilation', link: '/specifications/modules-and-compilation-specification/' },
            { label: 'Runtime capabilities, effects and sequencing', link: '/specifications/runtime-capabilities-effects-and-sequencing/' },
            { label: 'Concurrency', link: '/specifications/concurrency-specification/' },
            { label: 'Lazy computation and reclamation', link: '/specifications/lazy-computation-and-reclamation-specification/' },
            { label: 'Platform proofs and SIMD', link: '/specifications/platform-proofs-and-simd-specification/' },
            { label: 'Foreign function interface', link: '/specifications/foreign-function-interface-specification/' },
            { label: 'WebAssembly and WASI', link: '/specifications/webassembly-and-wasi-specification/' },
            { label: 'Runtime evolution', link: '/specifications/runtime-evolution-specification/' },
            { label: 'Attribute annotations and branch weights', link: '/specifications/attribute-annotations-specification/' },
          ],
        },
        {
          label: 'Tutorials',
          items: [
            { label: '1. Values and dependent types', link: '/tutorials/01-values-and-dependent-types/' },
            { label: '2. Proofs and runtime checks', link: '/tutorials/02-proofs-and-runtime-checks/' },
            { label: '3. Ownership and effects', link: '/tutorials/03-ownership-and-effects/' },
            { label: '4. Modules and laws', link: '/tutorials/04-modules-and-laws/' },
            { label: '5. Streams and coinduction', link: '/tutorials/05-streams-and-coinduction/' },
            { label: '6. Processes and temporal contracts', link: '/tutorials/06-processes-and-temporal-contracts/' },
            { label: '7. Rewrites and stream fusion', link: '/tutorials/07-rewrites-and-stream-fusion/' },
            { label: '8. Staging and schema generation', link: '/tutorials/08-staging-and-schema-generation/' },
            { label: '9. Compatible hot upgrades', link: '/tutorials/09-compatible-hot-upgrades/' },
            { label: '10. Platform specific SIMD', link: '/tutorials/10-platform-specific-simd/' },
            { label: '11. Foreign libraries', link: '/tutorials/11-foreign-libraries/' },
            { label: '12. WebAssembly components', link: '/tutorials/12-webassembly-components/' },
            { label: '13. Structured concurrency', link: '/tutorials/13-structured-concurrency/' },
            { label: '14. Haskell-like syntax, effects and runtime selection', link: '/tutorials/14-haskell-like-syntax-effects-and-runtime-selection/' },
          ],
        },
        {
          label: 'Design and references',
          items: [
            { label: 'Open decisions and implementation plan', link: '/design/open-decisions-and-implementation-plan/' },
            { label: 'References and related work', link: '/design/references-and-related-work/' },
            { label: 'Design conversation (historical record)', link: '/context/design-conversation/' },
            {
              label: 'Slice design notes',
              items: [
                { label: 'Attribute annotations', link: '/design/attribute-annotations/' },
                { label: 'Codata and guardedness', link: '/design/codata-guardedness/' },
                { label: 'Compiled interface metadata', link: '/design/compiled-interface-metadata/' },
                { label: 'Dependent core kernel', link: '/design/dependent-core-kernel/' },
                { label: 'Effects, capabilities, sequencing', link: '/design/effects-capabilities-sequencing/' },
                { label: 'Ownership and usage', link: '/design/ownership-and-usage/' },
              ],
            },
          ],
        },
        {
          label: 'Examples',
          items: [{ label: 'Example catalogue', link: '/examples/catalogue/' }],
        },
      ],
    }),
  ],
});
