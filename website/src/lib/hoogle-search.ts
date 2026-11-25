/**
 * Hoogle-style search engine for Cinch's library API index.
 *
 * Supports:
 * - Type signature queries: `(a -> b) -> List a -> List b`, `Result e a -> Option a`,
 *   `Ord a => List a -> List a`, `Bytes -> Result`, `->{IO r}`
 * - Combined queries: `map :: (a -> b) -> List a -> List b`
 * - Name & text queries: `partition-results`, `base64 decode`
 * - Module scoping: `+Data.List`, `module:Text.Json`, `-Crypto`
 * - Kind filters: `is:type`, `is:value`, `is:class`, `is:method`, `is:ctor`, `is:instance`
 */

export type IndexKindCode = 't' | 'k' | 'c' | 'm' | 'v' | 'i';

export type CompactSearchEntry = {
  /** Module name, e.g. `Data.Result` */
  m: string;
  /** Symbol name, e.g. `either` */
  n: string;
  /** Kind code: t=type, k=constructor, c=signature/class, m=method, v=value, i=instance */
  k: IndexKindCode;
  /** Pretty-printed signature */
  s: string;
  /** One-line plain summary */
  d: string;
  /** Anchor fragment inside `/packages/stdlib/<m>/`, e.g. `v-either` */
  h: string;
  /** Source line number (0 if unknown) */
  l: number;
  /** Source module if different from `m` (e.g. `Data.Core`) */
  sm?: string;
};

export type SearchIndexPayload = {
  schema: number;
  package?: string;
  moduleCount: number;
  typeLinks: Record<string, string>;
  entries: CompactSearchEntry[];
};

export type SearchFilters = {
  kind?: string;
};

export type SearchHit = {
  entry: CompactSearchEntry;
  score: number;
  mode: 'type' | 'name' | 'combined' | 'filter';
};

export type TypeExpr =
  | { tag: 'forall'; body: TypeExpr }
  | { tag: 'qual'; constraints: TypeExpr[]; body: TypeExpr }
  | { tag: 'arrow'; from: TypeExpr; to: TypeExpr; effects?: string }
  | { tag: 'tuple'; elems: TypeExpr[] }
  | { tag: 'app'; elems: TypeExpr[] }
  | { tag: 'con'; name: string }
  | { tag: 'var'; name: string }
  | { tag: 'raw'; text: string };

type Tok =
  | { t: '(' | ')' | ',' | '.' | '::' | '=>' | 'forall' | 'using' | 'Sigma' | 'refine' | 'with' }
  | { t: '->'; effects?: string }
  | { t: 'mult'; v: string }
  | { t: 'ident'; v: string }
  | { t: 'other'; v: string };

export function shortName(name: string): string {
  const idx = name.lastIndexOf('.');
  return idx >= 0 ? name.slice(idx + 1) : name;
}

function tokenizeType(input: string): Tok[] {
  const out: Tok[] = [];
  const len = input.length;
  let i = 0;

  while (i < len) {
    const ch = input[i]!;
    if (/\s/.test(ch)) {
      i++;
      continue;
    }
    if (ch === '(' || ch === ')' || ch === ',' || ch === '.') {
      out.push({ t: ch });
      i++;
      continue;
    }
    if (ch === ':' && input[i + 1] === ':') {
      out.push({ t: '::' });
      i += 2;
      continue;
    }
    if (ch === '=' && input[i + 1] === '>') {
      out.push({ t: '=>' });
      i += 2;
      continue;
    }
    if (ch === '-' && input[i + 1] === '>') {
      i += 2;
      if (input[i] === '{') {
        const start = i + 1;
        let depth = 1;
        i++;
        while (i < len && depth > 0) {
          if (input[i] === '{') depth++;
          else if (input[i] === '}') depth--;
          i++;
        }
        const end = depth === 0 ? i - 1 : i;
        out.push({ t: '->', effects: input.slice(start, end).trim() });
      } else {
        out.push({ t: '->' });
      }
      continue;
    }
    if (ch === '%') {
      const start = i++;
      while (i < len && /[A-Za-z0-9_]/.test(input[i]!)) i++;
      out.push({ t: 'mult', v: input.slice(start, i) });
      continue;
    }
    if (/[A-Za-z_]/.test(ch)) {
      const start = i++;
      const startsUpper = /[A-Z]/.test(ch);
      while (i < len) {
        const c = input[i]!;
        if (/[A-Za-z0-9_']/.test(c)) {
          i++;
        } else if (c === '-' && i + 1 < len && /[A-Za-z0-9_]/.test(input[i + 1]!)) {
          i++;
        } else if (startsUpper && c === '.' && i + 1 < len && /[A-Za-z_]/.test(input[i + 1]!)) {
          i++;
        } else {
          break;
        }
      }
      const word = input.slice(start, i);
      if (word === 'forall' || word === 'using' || word === 'Sigma' || word === 'refine' || word === 'with') {
        out.push({ t: word });
      } else {
        out.push({ t: 'ident', v: word });
      }
      continue;
    }
    const start = i++;
    while (i < len && !/[\s(),.{}\[\]A-Za-z_]/.test(input[i]!)) {
      if ((input[i] === '-' && input[i + 1] === '>') || (input[i] === '=' && input[i + 1] === '>') || (input[i] === ':' && input[i + 1] === ':')) {
        break;
      }
      i++;
    }
    out.push({ t: 'other', v: input.slice(start, i) });
  }
  return out;
}

class TypeParser {
  private pos = 0;
  private tokens: Tok[];
  constructor(tokens: Tok[]) {
    this.tokens = tokens;
  }

  private peek(offset = 0): Tok | undefined {
    return this.tokens[this.pos + offset];
  }

  private next(): Tok | undefined {
    return this.tokens[this.pos++];
  }

  parse(): TypeExpr | null {
    const res = this.parseType();
    if (!res || this.pos < this.tokens.length) return null;
    return res;
  }

  private parseType(): TypeExpr | null {
    const p = this.peek();
    if (!p) return null;
    if (p.t === 'forall') return this.parseForall();
    if (p.t === 'using') return this.parseUsing();
    if (p.t === 'Sigma') return this.parseSigma();
    if (p.t === 'refine') return this.parseRefine();
    return this.parseQualOrArrow();
  }

  private parseForall(): TypeExpr | null {
    this.next();
    while (this.peek()) {
      const p = this.peek()!;
      if (p.t === '.') {
        this.next();
        break;
      }
      if (p.t === '(') {
        let depth = 1;
        this.next();
        while (this.peek() && depth > 0) {
          const t = this.next()!;
          if (t.t === '(') depth++;
          else if (t.t === ')') depth--;
        }
        continue;
      }
      if (p.t === 'ident') {
        this.next();
        continue;
      }
      break;
    }
    const body = this.parseType();
    return body ? { tag: 'forall', body } : null;
  }

  private parseUsing(): TypeExpr | null {
    this.next();
    const constraints: TypeExpr[] = [];
    if (this.peek()?.t === '(') {
      this.next();
      while (this.peek() && this.peek()!.t !== ')') {
        const p0 = this.peek(0);
        const p1 = this.peek(1);
        if (p0?.t === 'ident' && p1?.t === '::') {
          this.next();
          this.next();
        }
        const ty = this.parseType();
        if (ty) constraints.push(ty);
        if (this.peek()?.t === ',') this.next();
        else break;
      }
      if (this.peek()?.t === ')') this.next();
    }
    if (this.peek()?.t === '=>') this.next();
    const body = this.parseType();
    if (!body) return null;
    return constraints.length ? { tag: 'qual', constraints, body } : body;
  }

  private parseSigma(): TypeExpr | null {
    this.next();
    if (this.peek()?.t === '(') {
      let depth = 1;
      this.next();
      while (this.peek() && depth > 0) {
        const t = this.next()!;
        if (t.t === '(') depth++;
        else if (t.t === ')') depth--;
      }
    }
    if (this.peek()?.t === '.') this.next();
    return this.parseType();
  }

  private parseRefine(): TypeExpr | null {
    this.next();
    let carrier: TypeExpr | null = null;
    if (this.peek()?.t === '(') {
      this.next();
      if (this.peek()?.t === 'ident' && this.peek(1)?.t === '::') {
        this.next();
        this.next();
      }
      carrier = this.parseType();
      if (this.peek()?.t === ')') this.next();
    }
    // Skip `with <predicate>` until end or enclosing `)`
    if (this.peek()?.t === 'with') {
      this.next();
      let depth = 0;
      while (this.peek()) {
        const t = this.peek()!;
        if (depth === 0 && (t.t === ')' || t.t === ',')) break;
        this.next();
        if (t.t === '(') depth++;
        else if (t.t === ')') depth = Math.max(0, depth - 1);
      }
    }
    return carrier ?? { tag: 'var', name: 'a' };
  }

  private parseQualOrArrow(): TypeExpr | null {
    // Handle dependent `(x :: T) -> ...`
    if (this.peek()?.t === '(' && this.peek(1)?.t === 'ident' && this.peek(2)?.t === '::') {
      const save = this.pos;
      this.next();
      this.next();
      this.next();
      const from = this.parseType();
      if (from && this.peek()?.t === ')') {
        this.next();
        if (this.peek()?.t === 'mult') this.next();
        const arr = this.peek();
        if (arr?.t === '->') {
          this.next();
          const to = this.parseType();
          if (to) return { tag: 'arrow', from, to, effects: arr.effects };
        }
      }
      this.pos = save;
    }

    const lhs = this.parseApp();
    if (!lhs) return null;

    if (this.peek()?.t === '=>') {
      this.next();
      const body = this.parseType();
      if (!body) return null;
      const constraints = lhs.tag === 'tuple' ? lhs.elems : [lhs];
      return { tag: 'qual', constraints, body };
    }

    if (this.peek()?.t === 'mult') this.next();

    const arr = this.peek();
    if (arr?.t === '->') {
      this.next();
      const rhs = this.parseType();
      if (!rhs) return null;
      return { tag: 'arrow', from: lhs, to: rhs, effects: arr.effects };
    }
    return lhs;
  }

  private parseApp(): TypeExpr | null {
    const items: TypeExpr[] = [];
    while (true) {
      const atom = this.parseAtom();
      if (!atom) break;
      items.push(atom);
    }
    if (items.length === 0) return null;
    if (items.length === 1) return items[0]!;
    return { tag: 'app', elems: items };
  }

  private parseAtom(): TypeExpr | null {
    const p = this.peek();
    if (!p) return null;
    if (p.t === '(') {
      this.next();
      if (this.peek()?.t === ')') {
        this.next();
        return { tag: 'con', name: 'Unit' };
      }
      const first = this.parseType();
      if (!first) return null;
      if (this.peek()?.t === ',') {
        const elems = [first];
        while (this.peek()?.t === ',') {
          this.next();
          const next = this.parseType();
          if (!next) return null;
          elems.push(next);
        }
        if (this.peek()?.t === ')') this.next();
        return { tag: 'tuple', elems };
      }
      if (this.peek()?.t === ')') this.next();
      return first;
    }
    if (p.t === 'ident') {
      this.next();
      return /^[A-Z]/.test(p.v) ? { tag: 'con', name: p.v } : { tag: 'var', name: p.v };
    }
    if (p.t === 'other') {
      this.next();
      return { tag: 'raw', text: p.v };
    }
    return null;
  }
}

export function parseTypeExpr(input: string): TypeExpr | null {
  const trimmed = input.trim();
  if (!trimmed) return null;
  return new TypeParser(tokenizeType(trimmed)).parse();
}

type Flattened = {
  constraints: TypeExpr[];
  args: TypeExpr[];
  effects: string[];
  ret: TypeExpr;
};

function flattenType(expr: TypeExpr): Flattened {
  const constraints: TypeExpr[] = [];
  const args: TypeExpr[] = [];
  const effects: string[] = [];
  let cur: TypeExpr = expr;
  while (true) {
    if (cur.tag === 'forall') {
      cur = cur.body;
    } else if (cur.tag === 'qual') {
      constraints.push(...cur.constraints);
      cur = cur.body;
    } else if (cur.tag === 'arrow') {
      args.push(cur.from);
      if (cur.effects) {
        for (const piece of cur.effects.split(/[,|]/)) {
          const trimmed = piece.trim();
          if (trimmed) effects.push(trimmed);
        }
      }
      cur = cur.to;
    } else {
      break;
    }
  }
  return { constraints, args, effects, ret: cur };
}

type MatchEnv = {
  qToC: Map<string, string>;
  cToQ: Map<string, string>;
};

function cloneEnv(env: MatchEnv): MatchEnv {
  return { qToC: new Map(env.qToC), cToQ: new Map(env.cToQ) };
}

function keyOf(e: TypeExpr): string {
  switch (e.tag) {
    case 'con':
      return `C:${shortName(e.name)}`;
    case 'var':
      return `V:${e.name}`;
    case 'raw':
      return `R:${e.text}`;
    case 'tuple':
      return `(${e.elems.map(keyOf).join(',')})`;
    case 'app':
      return `[${e.elems.map(keyOf).join(' ')}]`;
    case 'arrow':
      return `${keyOf(e.from)}->${keyOf(e.to)}`;
    case 'forall':
      return keyOf(e.body);
    case 'qual':
      return keyOf(e.body);
  }
}

function typeDistance(q: TypeExpr, c: TypeExpr, env: MatchEnv): number | null {
  if (q.tag === 'forall') return typeDistance(q.body, c, env);
  if (c.tag === 'forall') return typeDistance(q, c.body, env);

  if (q.tag === 'var' && c.tag === 'var') {
    const qb = env.qToC.get(q.name);
    if (qb !== undefined) return qb === `V:${c.name}` ? 0 : 18;
    const cb = env.cToQ.get(c.name);
    if (cb !== undefined) return cb === `V:${q.name}` ? 0 : 18;
    env.qToC.set(q.name, `V:${c.name}`);
    env.cToQ.set(c.name, `V:${q.name}`);
    return 0;
  }

  if (q.tag === 'var') {
    const extra = c.tag === 'arrow' ? 6 : c.tag === 'app' || c.tag === 'tuple' ? 4 : 0;
    const ck = keyOf(c);
    const qb = env.qToC.get(q.name);
    if (qb !== undefined) return qb === ck ? 4 : null;
    env.qToC.set(q.name, ck);
    return 6 + extra;
  }

  if (c.tag === 'var') {
    const extra = q.tag === 'arrow' ? 22 : q.tag === 'app' || q.tag === 'tuple' ? 16 : 0;
    const qk = keyOf(q);
    const cb = env.cToQ.get(c.name);
    if (cb !== undefined) return cb === qk ? 8 : null;
    env.cToQ.set(c.name, qk);
    return 12 + extra;
  }

  if (q.tag === 'con' && c.tag === 'con') {
    const qs = shortName(q.name);
    const cs = shortName(c.name);
    if (qs === cs) return 0;
    if (qs.toLowerCase() === cs.toLowerCase()) return 10;
    return null;
  }

  if (q.tag === 'con' && c.tag === 'app' && c.elems[0]?.tag === 'con') {
    if (shortName(q.name).toLowerCase() === shortName(c.elems[0].name).toLowerCase()) {
      return 10 + (c.elems.length - 1) * 3;
    }
    return null;
  }

  if (q.tag === 'tuple' && c.tag === 'tuple') {
    if (q.elems.length !== c.elems.length) return null;
    let total = 0;
    for (let i = 0; i < q.elems.length; i++) {
      const d = typeDistance(q.elems[i]!, c.elems[i]!, env);
      if (d === null) return null;
      total += d;
    }
    return total;
  }

  if (q.tag === 'app' && c.tag === 'app') {
    if (q.elems.length !== c.elems.length) return null;
    let total = 0;
    for (let i = 0; i < q.elems.length; i++) {
      const d = typeDistance(q.elems[i]!, c.elems[i]!, env);
      if (d === null) return null;
      total += d;
    }
    return total;
  }

  if (q.tag === 'arrow' && c.tag === 'arrow') {
    const fq = flattenType(q);
    const fc = flattenType(c);
    if (fq.args.length !== fc.args.length) return null;
    const rd = typeDistance(fq.ret, fc.ret, env);
    if (rd === null) return null;
    let total = rd;
    for (let i = 0; i < fq.args.length; i++) {
      const ad = typeDistance(fq.args[i]!, fc.args[i]!, env);
      if (ad === null) return null;
      total += ad;
    }
    return total;
  }

  return null;
}

export function scoreTypeMatch(query: TypeExpr, candidate: TypeExpr): number | null {
  const fq = flattenType(query);
  const fc = flattenType(candidate);

  if (fq.args.length === 0) {
    const env: MatchEnv = { qToC: new Map(), cToQ: new Map() };
    const retDist = typeDistance(fq.ret, fc.ret, env);
    if (retDist !== null) {
      return Math.max(50, 960 - retDist * 8 - fc.args.length * 14);
    }
    let best: number | null = null;
    for (const arg of fc.args) {
      const argEnv: MatchEnv = { qToC: new Map(), cToQ: new Map() };
      const d = typeDistance(fq.ret, arg, argEnv);
      if (d !== null) {
        const s = Math.max(40, 760 - d * 8 - fc.args.length * 10);
        if (best === null || s > best) best = s;
      }
    }
    return best;
  }

  if (fc.args.length < fq.args.length || fc.args.length > fq.args.length + 2) {
    return null;
  }

  const baseEnv: MatchEnv = { qToC: new Map(), cToQ: new Map() };
  const retDist = typeDistance(fq.ret, fc.ret, baseEnv);
  if (retDist === null) return null;

  let bestArgCost: number | null = null;
  if (fq.args.length === fc.args.length) {
    const env = cloneEnv(baseEnv);
    let ok = true;
    let cost = 0;
    for (let i = 0; i < fq.args.length; i++) {
      const d = typeDistance(fq.args[i]!, fc.args[i]!, env);
      if (d === null) {
        ok = false;
        break;
      }
      cost += d;
    }
    if (ok) bestArgCost = cost;
  }

  if (bestArgCost === null || bestArgCost > 0) {
    let env = cloneEnv(baseEnv);
    const used = new Array<boolean>(fc.args.length).fill(false);
    let total = 0;
    let allMatched = true;
    let lastIdx = -1;

    for (const qa of fq.args) {
      let pick: { ci: number; d: number; nextEnv: MatchEnv } | null = null;
      for (let ci = 0; ci < fc.args.length; ci++) {
        if (used[ci]) continue;
        const trial = cloneEnv(env);
        const d = typeDistance(qa, fc.args[ci]!, trial);
        if (d !== null) {
          const orderPenalty = lastIdx >= 0 && ci < lastIdx ? 14 : 0;
          const totalD = d + orderPenalty;
          if (!pick || totalD < pick.d) {
            pick = { ci, d: totalD, nextEnv: trial };
          }
        }
      }
      if (pick) {
        used[pick.ci] = true;
        total += pick.d;
        lastIdx = pick.ci;
        env = pick.nextEnv;
      } else {
        allMatched = false;
        break;
      }
    }
    if (allMatched && (bestArgCost === null || total < bestArgCost)) {
      bestArgCost = total;
    }
  }

  if (bestArgCost === null) return null;

  const extraPenalty = (fc.args.length - fq.args.length) * 45;
  let constraintDelta = 0;
  if (fq.constraints.length > 0) {
    let matched = 0;
    for (const qc of fq.constraints) {
      if (fc.constraints.some((cc) => typeDistance(qc, cc, { qToC: new Map(), cToQ: new Map() }) !== null)) {
        matched++;
      }
    }
    constraintDelta = matched === fq.constraints.length ? 25 : -(fq.constraints.length - matched) * 35;
  } else if (fc.constraints.length > 0) {
    constraintDelta = -fc.constraints.length * 3;
  }

  let effectDelta = 0;
  if (fq.effects.length > 0) {
    const cHeads = new Set(fc.effects.map((e) => e.split(/\s+/)[0]!.toLowerCase()));
    for (const qe of fq.effects) {
      const h = qe.split(/\s+/)[0]!.toLowerCase();
      effectDelta += cHeads.has(h) ? 20 : -40;
    }
  } else if (fc.effects.length > 0) {
    effectDelta = -6;
  }

  const score = 1000 - retDist * 10 - bestArgCost * 7 - extraPenalty + constraintDelta + effectDelta;
  return score >= 120 ? score : null;
}

const KIND_MAP = new Map<string, IndexKindCode[]>([
  ['type', ['t']],
  ['data', ['t']],
  ['record', ['t']],
  ['codata', ['t']],
  ['ctor', ['k']],
  ['constructor', ['k']],
  ['class', ['c']],
  ['signature', ['c']],
  ['sig', ['c']],
  ['method', ['m']],
  ['value', ['v']],
  ['fn', ['v']],
  ['func', ['v']],
  ['function', ['v']],
  ['instance', ['i']],
]);

function looksLikeTypeQuery(s: string): boolean {
  const t = s.trim();
  if (!t) return false;
  if (
    t.includes('->') ||
    t.includes('=>') ||
    t.startsWith('forall ') ||
    t.startsWith('using ') ||
    t.startsWith('Sigma ') ||
    t.startsWith('refine ') ||
    (t.startsWith('(') && t.includes(','))
  ) {
    return true;
  }
  const words = t.split(/\s+/);
  return words.length >= 2 && /^[A-Z]/.test(words[0]!);
}

function isSubsequence(needle: string, haystack: string): boolean {
  let ni = 0;
  for (let hi = 0; hi < haystack.length && ni < needle.length; hi++) {
    if (haystack[hi] === needle[ni]) ni++;
  }
  return ni === needle.length;
}

function scoreNameMatch(query: string, entry: CompactSearchEntry): number | null {
  const raw = query.trim();
  const q = raw.toLowerCase();
  if (!q) return null;

  const n = entry.n;
  const nlc = n.toLowerCase();
  const mlc = entry.m.toLowerCase();
  const idlc = `${mlc}.${nlc}`;
  const dlc = entry.d.toLowerCase();

  let score: number;
  if (n === raw) score = 1000;
  else if (`${entry.m}.${n}` === raw) score = 980;
  else if (nlc === q) score = 950;
  else if (idlc === q) score = 930;
  else if (nlc.startsWith(q)) score = 860 - Math.min(60, nlc.length - q.length);
  else if (nlc.split(/[-_.]/).some((seg) => seg.startsWith(q))) score = 800 - Math.min(60, nlc.length - q.length);
  else if (nlc.includes(q)) score = 730 - Math.min(80, nlc.length - q.length);
  else if (idlc.includes(q)) score = 660;
  else if (q.length >= 3 && isSubsequence(q, nlc)) score = 540 - Math.min(120, (nlc.length - q.length) * 3);
  else {
    const words = q.split(/\s+/).filter(Boolean);
    if (words.every((w) => nlc.includes(w) || mlc.includes(w) || dlc.includes(w))) {
      const nameHits = words.filter((w) => nlc.includes(w)).length;
      score = 440 + nameHits * 60;
    } else {
      return null;
    }
  }

  if (entry.m === 'Prelude') score += 12;
  else if (entry.m !== 'Data.Core' && !entry.m.includes('.Internal')) score += 6;
  return score;
}

function stripLeadingName(sig: string): string {
  const idx = sig.indexOf('::');
  if (idx < 0) return sig;
  const lhs = sig.slice(0, idx).trim();
  if (lhs && !lhs.includes(' ') && !lhs.startsWith('forall') && !lhs.startsWith('using')) {
    return sig.slice(idx + 2).trim();
  }
  return sig;
}

const parsedCache = new WeakMap<CompactSearchEntry, TypeExpr | null>();

function entryTypeAst(entry: CompactSearchEntry): TypeExpr | null {
  if (parsedCache.has(entry)) return parsedCache.get(entry) ?? null;
  const ast = parseTypeExpr(stripLeadingName(entry.s));
  parsedCache.set(entry, ast);
  return ast;
}

export function runHoogleSearch(
  entries: CompactSearchEntry[],
  rawQuery: string,
  filters: SearchFilters = {},
): SearchHit[] {
  const includeModules: string[] = [];
  const excludeModules: string[] = [];
  const kindFilters: IndexKindCode[] = [];
  const restTokens: string[] = [];

  if (filters.kind && filters.kind !== 'all') {
    const mapped = KIND_MAP.get(filters.kind.toLowerCase());
    if (mapped) kindFilters.push(...mapped);
  }

  for (const word of rawQuery.trim().split(/\s+/).filter(Boolean)) {
    if (word.startsWith('+') && word.length > 1) {
      includeModules.push(word.slice(1).toLowerCase());
    } else if (word.startsWith('-') && word.length > 1 && /[A-Za-z]/.test(word[1]!)) {
      excludeModules.push(word.slice(1).toLowerCase());
    } else if (word.startsWith('module:') && word.length > 7) {
      includeModules.push(word.slice(7).toLowerCase());
    } else if ((word.startsWith('is:') || word.startsWith('kind:')) && word.includes(':')) {
      const k = word.slice(word.indexOf(':') + 1).toLowerCase();
      const mapped = KIND_MAP.get(k);
      if (mapped) kindFilters.push(...mapped);
    } else {
      restTokens.push(word);
    }
  }

  const remainder = restTokens.join(' ');
  let nameQuery: string | null = null;
  let typeQuery: TypeExpr | null = null;

  const colonIdx = remainder.indexOf('::');
  if (colonIdx >= 0) {
    const lhs = remainder.slice(0, colonIdx).trim();
    const rhs = remainder.slice(colonIdx + 2).trim();
    if (lhs) nameQuery = lhs;
    if (rhs) typeQuery = parseTypeExpr(rhs);
  } else if (looksLikeTypeQuery(remainder)) {
    const parsed = parseTypeExpr(remainder);
    if (parsed) typeQuery = parsed;
    else if (remainder) nameQuery = remainder;
  } else if (remainder) {
    nameQuery = remainder;
  }

  const hasFilters = includeModules.length > 0 || excludeModules.length > 0 || kindFilters.length > 0;
  if (!nameQuery && !typeQuery && !hasFilters) {
    return [];
  }

  const hits: SearchHit[] = [];

  for (const entry of entries) {
    const mlc = entry.m.toLowerCase();
    if (includeModules.length > 0 && !includeModules.some((m) => mlc === m || mlc.startsWith(`${m}.`) || mlc.includes(m))) {
      continue;
    }
    if (excludeModules.some((m) => mlc === m || mlc.startsWith(`${m}.`))) {
      continue;
    }
    if (kindFilters.length > 0 && !kindFilters.includes(entry.k)) {
      continue;
    }

    if (!nameQuery && !typeQuery) {
      hits.push({ entry, score: 500, mode: 'filter' });
      continue;
    }

    const singleTypeWord =
      nameQuery && !typeQuery && /^[A-Z][A-Za-z0-9_.]*$/.test(nameQuery)
        ? parseTypeExpr(nameQuery)
        : null;

    const nameScore = nameQuery ? scoreNameMatch(nameQuery, entry) : null;
    const effectiveTypeQuery = typeQuery ?? singleTypeWord;
    const typeScore = effectiveTypeQuery
      ? (() => {
          const ast = entryTypeAst(entry);
          return ast ? scoreTypeMatch(effectiveTypeQuery, ast) : null;
        })()
      : null;

    if (nameQuery && typeQuery) {
      if (nameScore !== null && typeScore !== null) {
        hits.push({ entry, score: Math.floor((nameScore + typeScore) / 2) + 100, mode: 'combined' });
      }
    } else if (typeQuery) {
      if (typeScore !== null) {
        hits.push({ entry, score: typeScore, mode: 'type' });
      }
    } else if (nameQuery) {
      if (nameScore !== null || typeScore !== null) {
        const score = Math.max(nameScore ?? -1, typeScore !== null ? Math.floor(typeScore * 0.8) : -1);
        hits.push({ entry, score, mode: nameScore !== null ? 'name' : 'type' });
      }
    }
  }

  hits.sort((a, b) => b.score - a.score || a.entry.m.localeCompare(b.entry.m) || a.entry.n.localeCompare(b.entry.n));
  return hits;
}

export function kindLabel(k: IndexKindCode): string {
  switch (k) {
    case 't':
      return 'type';
    case 'k':
      return 'ctor';
    case 'c':
      return 'signature';
    case 'm':
      return 'method';
    case 'v':
      return 'value';
    case 'i':
      return 'instance';
  }
}
