import type {APIRoute} from 'astro';
import {artifacts} from '../../../lib/downloads';
export function getStaticPaths(){return artifacts.map(artifact=>({params:{artifact:artifact.id},props:{artifact}}));}
export const GET:APIRoute=({props})=>new Response(`${props.artifact.sha256}  ${props.artifact.filename}\n`,{headers:{'Content-Type':'text/plain; charset=utf-8'}});
