import type { APIRoute } from 'astro';
import { loadCatalogue, publicCatalogue } from '../../lib/catalogue';

export const GET: APIRoute = async () => {
  const catalogue = await loadCatalogue();
  const payload = publicCatalogue(catalogue);
  return new Response(JSON.stringify(payload, null, 2) + '\n', {
    headers: { 'Content-Type': 'application/json; charset=utf-8' },
  });
};
