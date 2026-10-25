import { defineCollection } from 'astro:content';
import { docsSchema } from '@astrojs/starlight/schema';
import { z } from 'astro/zod';
import { safelangDocs } from './loaders/safelang-docs';

export const collections = {
  docs: defineCollection({
    loader: safelangDocs(),
    schema: docsSchema({
      extend: z.object({
        /** Where the page came from: an original export URL or a note that it is a local draft. */
        source: z.string().optional(),
        /** Export timestamp, present on pages taken from the documentation export. */
        exported: z.string().optional(),
        /** Authoring date, present on pages written in this repository. */
        added: z.string().optional(),
      }),
    }),
  }),
};
