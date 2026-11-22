import { strict as assert } from 'node:assert';
import { readFile } from 'node:fs/promises';
import { ConsoleStdout, File, OpenFile, WASI } from '@bjorn3/browser_wasi_shim';
import { Life, height, type BoardExports } from '../src/lib/life.ts';

const assets = new URL('../src/data/examples/', import.meta.url);
const read = (name: string) => readFile(new URL(name, assets));
const board = await WebAssembly.instantiate(await read('life-board.wasm'));
const life = new Life(board.instance.exports as BoardExports);

const wasi = new WASI([], [], [
  new OpenFile(new File([])),
  ConsoleStdout.lineBuffered(() => {}),
  ConsoleStdout.lineBuffered(() => {}),
]);
const { instance } = await WebAssembly.instantiate(await read('life-compiler.wasm'), {
  wasi_snapshot_preview1: wasi.wasiImport,
});
const wasm = instance.exports as WebAssembly.Exports & {
  memory: WebAssembly.Memory;
  life_input(): number;
  life_compile(length: number): number;
  life_output(): number;
};
wasi.initialize({ exports: wasm });

function compile(source: string): { table: number } | { error: string } {
  const input = new TextEncoder().encode(source);
  new Uint8Array(wasm.memory.buffer, wasm.life_input(), input.length).set(input);
  const length = wasm.life_compile(input.length);
  const bytes = new Uint8Array(wasm.memory.buffer, wasm.life_output(), length);
  return JSON.parse(new TextDecoder().decode(bytes));
}

const lines = (await read('life.cinch')).toString().split('\n');
const start = lines.findIndex((line) => line.trim().startsWith('-- What a cell becomes'));
const end = lines.findIndex((line, index) => index > start && line.trim() === '');
const original = lines.slice(start, end).map((line) => line.replace(/^  /, '')).join('\n');
assert.deepEqual(compile(original), { table: 6152 });
assert.equal((board.instance.exports as BoardExports).life_initial_rule(), 6152);

// A horizontal blinker near the top left, with dead cells beyond the edge.
life.state.fill(0n);
life.state[1] = 7n;
life.step();
assert.deepEqual([...life.state.slice(0, 4)], [2n, 2n, 2n, 0n]);
assert.equal(life.state[height], 1n);
const snapshot = life.state.slice();

const invalid = compile(original.replace('else Dead', 'else 99'));
assert.ok('error' in invalid && invalid.error.includes('type Cell'));
const signature = compile('rule :: Cell -> Bool -> Cell\nrule cell n = cell');
assert.ok('error' in signature);
assert.deepEqual(life.state, snapshot);

// Use a binding and arithmetic, which the old rule reader could not compile.
const next = compile('rule :: Cell -> Int -> Cell\nrule cell n = let k = n + 1 in if k >= 1 then Alive else Dead');
assert.ok('table' in next);
assert.equal(life.install(next.table), 2);
assert.deepEqual(life.state, snapshot);
life.step();
assert.ok(life.state.slice(0, height).every((row) => row === (1n << 40n) - 1n));
assert.equal(life.state[height], 2n);
assert.deepEqual(compile(original), { table: 6152 });
console.log('Cinch Wasm checked edits, preserved the board and generation, and ran the new rule.');
