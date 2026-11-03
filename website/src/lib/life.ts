// The landing page's Game of Life: a board that steps exactly like `step` in
// src/data/examples/life.cinch (bounded, cells past the edge are dead), and a
// reader for edits to its `rule` declaration.
//
// The reader stands in for the compiler until the browser build exists. It
// accepts a subset of Cinch that the Rust compiler also accepts: a `case` on the
// cell whose arms are `Alive`, `Dead` or `_`, each an `if` chain over
// comparisons of the neighbour count with integer literals, joined by `&&`,
// `||` and `not (...)`. Anything else is reported as unreadable rather than
// guessed at.

export type Cell = 0 | 1;
export type Rule = (cell: Cell, neighbours: number) => Cell;

export type Load =
  | { ok: true; rule: Rule }
  | { ok: false; reason: 'interface' }
  | { ok: false; reason: 'missing'; constructor: 'Alive' | 'Dead' }
  | { ok: false; reason: 'unreadable' };

export const SIGNATURE = 'rule :: Cell -> Int -> Cell';

type Token = { kind: 'var' | 'con' | 'int' | 'op' | 'kw'; text: string };
const KEYWORDS: Record<string, true> = { case: true, of: true, if: true, then: true, else: true, not: true };
const TOKEN = /\s*(?:(\d+)|([a-z][A-Za-z0-9_'-]*)|([A-Z][A-Za-z0-9_]*)|(==|\/=|<=|>=|&&|\|\||->|::|[<>=()_]))/y;

class Unreadable extends Error {}

function lex(text: string): Token[] {
  const tokens: Token[] = [];
  TOKEN.lastIndex = 0;
  while (TOKEN.lastIndex < text.length) {
    if (/^\s*$/.test(text.slice(TOKEN.lastIndex))) break;
    const m = TOKEN.exec(text);
    if (!m) throw new Unreadable();
    if (m[1]) tokens.push({ kind: 'int', text: m[1] });
    else if (m[2]) tokens.push({ kind: Object.hasOwn(KEYWORDS, m[2]) ? 'kw' : 'var', text: m[2] });
    else if (m[3]) tokens.push({ kind: 'con', text: m[3] });
    else tokens.push({ kind: 'op', text: m[4] });
  }
  return tokens;
}

type Bool = (n: number) => boolean;
type CellExpr = (n: number) => Cell;

function parse(tokens: Token[]): { cellVar: string; arms: { pattern: string; body: CellExpr }[] } {
  let i = 0;
  const take = (text?: string, kind?: Token['kind']) => {
    const t = tokens[i];
    if (!t || (text !== undefined && t.text !== text) || (kind !== undefined && t.kind !== kind)) throw new Unreadable();
    i++;
    return t;
  };

  take('rule');
  const cellVar = take(undefined, 'var').text;
  const countVar = take(undefined, 'var').text;
  take('=');
  take('case');
  if (take(undefined, 'var').text !== cellVar) throw new Unreadable();
  take('of');

  const count = () => {
    const t = take();
    if (t.kind === 'int') { const v = Number(t.text); return () => v; }
    if (t.kind === 'var' && t.text === countVar) return (n: number) => n;
    throw new Unreadable();
  };
  const comparison = (): Bool => {
    const left = count();
    const op = take(undefined, 'op').text;
    const right = count();
    switch (op) {
      case '==': return (n) => left(n) === right(n);
      case '/=': return (n) => left(n) !== right(n);
      case '<': return (n) => left(n) < right(n);
      case '<=': return (n) => left(n) <= right(n);
      case '>': return (n) => left(n) > right(n);
      case '>=': return (n) => left(n) >= right(n);
      default: throw new Unreadable();
    }
  };
  const atom = (): Bool => {
    const t = tokens[i];
    if (t?.text === '(') { take('('); const b = disjunction(); take(')'); return b; }
    if (t?.text === 'True') { take(); return () => true; }
    if (t?.text === 'False') { take(); return () => false; }
    if (t?.text === 'not') {
      take('not');
      // `not n == 2` is `(not n) == 2` in Cinch, a type error, so `not` needs a closed operand.
      const next = tokens[i];
      if (next?.text !== '(' && next?.text !== 'True' && next?.text !== 'False') throw new Unreadable();
      const b = atom();
      return (n) => !b(n);
    }
    return comparison();
  };
  const conjunction = (): Bool => {
    let b = atom();
    while (tokens[i]?.text === '&&') { take(); const l = b, r = atom(); b = (n) => l(n) && r(n); }
    return b;
  };
  const disjunction = (): Bool => {
    let b = conjunction();
    while (tokens[i]?.text === '||') { take(); const l = b, r = conjunction(); b = (n) => l(n) || r(n); }
    return b;
  };
  const cellExpr = (): CellExpr => {
    const t = tokens[i];
    if (t?.text === 'if') {
      take('if');
      const test = disjunction();
      take('then');
      const yes = cellExpr();
      take('else');
      const no = cellExpr();
      return (n) => (test(n) ? yes(n) : no(n));
    }
    if (t?.text === '(') { take('('); const e = cellExpr(); take(')'); return e; }
    if (t?.text === 'Alive') { take(); return () => 1; }
    if (t?.text === 'Dead') { take(); return () => 0; }
    throw new Unreadable();
  };

  const arms: { pattern: string; body: CellExpr }[] = [];
  while (i < tokens.length) {
    const pattern = take().text;
    if (pattern !== 'Alive' && pattern !== 'Dead' && pattern !== '_') throw new Unreadable();
    take('->');
    arms.push({ pattern, body: cellExpr() });
  }
  if (arms.length === 0) throw new Unreadable();
  return { cellVar, arms };
}

/** Read an edited `rule` declaration (comments allowed) into a rule the board can run. */
export function loadRule(source: string): Load {
  const lines = source.split('\n').map((l) => l.replace(/--.*$/, ''));
  const signatureAt = lines.findIndex((l) => /::/.test(l));
  if (signatureAt < 0 || lines[signatureAt].trim().replace(/\s+/g, ' ') !== SIGNATURE) return { ok: false, reason: 'interface' };
  const body = lines.filter((_, i) => i !== signatureAt).join('\n');
  let parsed;
  try {
    const tokens = lex(body);
    // A renamed declaration or a different parameter count changes what `step` calls.
    if (tokens[0]?.text !== 'rule') return { ok: false, reason: 'interface' };
    const equals = tokens.findIndex((t) => t.text === '=');
    if (equals !== 3 || tokens.slice(1, 3).some((t) => t.kind !== 'var')) return { ok: false, reason: 'interface' };
    parsed = parse(tokens);
  } catch (e) {
    if (e instanceof Unreadable) return { ok: false, reason: 'unreadable' };
    throw e;
  }
  const { arms } = parsed;
  for (const constructor of ['Alive', 'Dead'] as const) {
    if (!arms.some((a) => a.pattern === constructor || a.pattern === '_')) return { ok: false, reason: 'missing', constructor };
  }
  return {
    ok: true,
    rule: (cell, n) => {
      const name = cell ? 'Alive' : 'Dead';
      const arm = arms.find((a) => a.pattern === name || a.pattern === '_')!;
      return arm.body(n);
    },
  };
}

/** One generation, as `step` computes it: cells beyond the edge count as dead. */
export function step(board: Uint8Array, width: number, height: number, rule: Rule): Uint8Array {
  const next = new Uint8Array(board.length);
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      let n = 0;
      for (let dy = -1; dy <= 1; dy++) {
        const yy = y + dy;
        if (yy < 0 || yy >= height) continue;
        for (let dx = -1; dx <= 1; dx++) {
          const xx = x + dx;
          if ((dx || dy) && xx >= 0 && xx < width) n += board[yy * width + xx];
        }
      }
      next[y * width + x] = rule(board[y * width + x] as Cell, n);
    }
  }
  return next;
}

/** Gosper's glider gun, placed with its top-left corner at (left, top). */
export function gliderGun(width: number, height: number, left = 1, top = 1): Uint8Array {
  const rows = [
    '........................#...........',
    '......................#.#...........',
    '............##......##............##',
    '...........#...#....##............##',
    '##........#.....#...##..............',
    '##........#...#.##....#.#...........',
    '..........#.....#.......#...........',
    '...........#...#....................',
    '............##......................',
  ];
  const board = new Uint8Array(width * height);
  rows.forEach((row, y) => [...row].forEach((c, x) => {
    if (c === '#' && y + top < height && x + left < width) board[(y + top) * width + x + left] = 1;
  }));
  return board;
}
