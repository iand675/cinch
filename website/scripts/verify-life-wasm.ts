import { strict as assert } from 'node:assert';
import { readFile } from 'node:fs/promises';
import { ConsoleStdout, File, OpenFile, WASI } from '@bjorn3/browser_wasi_shim';
import { Life, maxAge, height, width, type BoardExports, type LifeSettings } from '../src/lib/life.ts';

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

function compile(source: string): LifeSettings | { error: string } {
  const input = new TextEncoder().encode(source);
  new Uint8Array(wasm.memory.buffer, wasm.life_input(), input.length).set(input);
  const length = wasm.life_compile(input.length);
  const bytes = new Uint8Array(wasm.memory.buffer, wasm.life_output(), length);
  return JSON.parse(new TextDecoder().decode(bytes));
}

const original = (await read('life.cinch')).toString();
const defaults = life.settings;
assert.equal(defaults.stepMs, 80);
assert.equal(defaults.colors.length, maxAge);
assert.equal(defaults.wrapEdges, true);
assert.deepEqual(compile(original), defaults);

// A blinker crosses the horizontal edge, then the vertical edge.
life.state.fill(0n);
life.ages.fill(0);
life.toggle(width - 1, 0);
life.toggle(0, 0);
life.toggle(1, 0);
life.step();
assert.deepEqual([...life.state.slice(0, 3)], [1n, 1n, 0n]);
assert.equal(life.state[height - 1], 1n);
assert.equal(life.state[height], 1n);
assert.equal(life.ages[0], 2);
assert.equal(life.ages[1], 0);
assert.equal(life.ages[width - 1], 0);
assert.equal(life.ages[width], 1);
assert.equal(life.ages[(height - 1) * width], 1);
const rows = life.state.slice();
const ages = life.ages.slice();

const invalidEdits = [
  [original.replace('then 1 else 0', 'then True else 0'), 'Line '],
  [original.replace('then 1 else 0', 'then 33 else 0'), 'next-age 0 3'],
  [original.replace('neighbour-count mask = count-bits mask', 'neighbour-count mask = 9'), 'neighbour-count 0'],
  [original.replace('0xffd69c', '16777216'), 'cell-color 1'],
  [original.replace('step-ms = 80', 'step-ms = 2147483648'), 'step-ms must be'],
];
for (const [source, message] of invalidEdits) {
  const invalid = compile(source);
  assert.ok('error' in invalid && invalid.error.includes(message), JSON.stringify(invalid));
}
assert.equal(life.settings, defaults);
assert.deepEqual(life.state, rows);
assert.deepEqual(life.ages, ages);

// An added declaration and changes to every part of the module take effect together.
const edited = original
  .replace('if n == 3 then 1 else 0', 'if birth-count n then 1 else 0')
  .replace('if age < 32 then age + 1 else age', 'age')
  .replace('neighbour-count mask = count-bits mask', 'neighbour-count mask = count-bits (bit-and-u32 mask 24)')
  .replace('wrap-edges = True', 'wrap-edges = False')
  .replace('0xffd69c', '0x00ffff')
  .replace('step-ms = 80', 'step-ms = 0')
  + '\n  birth-count :: Int -> Bool\n  birth-count n = n == 0\n';
const next = compile(edited);
assert.ok('ages' in next, JSON.stringify(next));
assert.equal(next.ages[0], 1);
assert.equal(next.ages[2 * 9 + 2], 2);
assert.equal(next.neighbours[255], 2);
assert.equal(next.wrapEdges, false);
assert.equal(next.colors[0], 0x00ffff);
assert.equal(next.stepMs, 0);
assert.equal(life.install(next), 2);
assert.deepEqual(life.settings, next);
assert.deepEqual(life.state, rows);
assert.deepEqual(life.ages, ages);

life.step();
assert.equal(life.state[height], 2n);
assert.equal(life.ages[0], 0);
assert.equal(life.ages[1], 0);
assert.equal(life.ages[2], 1);
assert.deepEqual(compile(original), defaults);
life.reset();
assert.deepEqual(life.settings, next);
assert.equal(life.state[height], 0n);

function checkAgesMatchRows() {
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const live = (life.state[y] >> BigInt(x)) & 1n;
      assert.equal(life.ages[y * width + x], Number(live));
    }
  }
}
checkAgesMatchRows();
life.reseed();
checkAgesMatchRows();

console.log('Cinch Wasm compiled Life edits, activated settings atomically, and preserved the board, ages, and generation.');
