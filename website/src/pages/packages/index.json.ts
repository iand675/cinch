import type {APIRoute} from 'astro';
import {packages} from '../../lib/catalogue';
export const GET:APIRoute=()=>new Response(JSON.stringify({schema:1,sample:true,packages},null,2)+'\n',{headers:{'Content-Type':'application/json; charset=utf-8'}});
