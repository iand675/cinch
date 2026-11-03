import type {APIRoute} from 'astro';
import {artifacts} from '../../../lib/downloads';
export function getStaticPaths(){return artifacts.map(artifact=>({params:{artifact:artifact.id},props:{artifact}}));}
export const GET:APIRoute=({props})=>new Response(new Uint8Array(props.artifact.bytes),{headers:{'Content-Type':'application/zip','Content-Disposition':`attachment; filename="${props.artifact.filename}"`}});
