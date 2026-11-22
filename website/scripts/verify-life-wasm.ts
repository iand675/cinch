import { strict as assert } from 'node:assert';
import { readFile } from 'node:fs/promises';
import { ConsoleStdout, File, OpenFile, WASI } from '@bjorn3/browser_wasi_shim';
import { Life, maxAge, type BoardExports, type LifeSettings } from '../src/lib/life.ts';

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
const { width, height } = defaults;
assert.equal(width, 40);
assert.equal(height, 24);
assert.equal(defaults.stepMs, 80);
assert.equal(defaults.colors.length, maxAge);
assert.equal(defaults.wrapEdges, true);
assert.deepEqual(compile(original), defaults);

// A blinker crosses the horizontal edge, then the vertical edge.
life.rows.fill(0n);
life.ages.fill(0);
life.toggle(width - 1, 0);
life.toggle(0, 0);
life.toggle(1, 0);
life.step();
assert.deepEqual([...life.rows.slice(0, 3)], [1n, 1n, 0n]);
assert.equal(life.rows[height - 1], 1n);
assert.equal(life.generation, 1n);
assert.equal(life.ages[0], 2);
assert.equal(life.ages[1], 0);
assert.equal(life.ages[width - 1], 0);
assert.equal(life.ages[width], 1);
assert.equal(life.ages[(height - 1) * width], 1);
const rows = life.rows.slice();
const ages = life.ages.slice();

const invalidEdits = [
  [original.replace('then 1 else 0', 'then True else 0'), 'Line '],
  [original.replace('then 1 else 0', 'then 33 else 0'), 'next-age 0 3'],
  [original.replace('neighbour-count mask = count-bits mask', 'neighbour-count mask = 9'), 'neighbour-count 0'],
  [original.replace('0xffd69c', '16777216'), 'cell-color 1'],
  [original.replace('step-ms = 80', 'step-ms = 2147483648'), 'step-ms must be'],
  [original.replace('grid-width = 40', 'grid-width = 63'), 'grid-width must be between 1 and 62'],
  [original.replace('grid-height = 24', 'grid-height = 0'), 'grid-height must be between 1 and 64'],
];
for (const [source, message] of invalidEdits) {
  const invalid = compile(source);
  assert.ok('error' in invalid && invalid.error.includes(message), JSON.stringify(invalid));
}
assert.equal(life.settings, defaults);
assert.deepEqual(life.rows, rows);
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
assert.deepEqual(life.rows, rows);
assert.deepEqual(life.ages, ages);

life.step();
assert.equal(life.generation, 2n);
assert.equal(life.ages[0], 0);
assert.equal(life.ages[1], 0);
assert.equal(life.ages[2], 1);
assert.deepEqual(compile(original), defaults);
life.reset();
assert.deepEqual(life.settings, next);
assert.equal(life.generation, 0n);

function checkAgesMatchRows() {
  const { width, height } = life.settings;
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const live = (life.rows[y] >> BigInt(x)) & 1n;
      assert.equal(life.ages[y * width + x] > 0, live === 1n);
    }
  }
}
checkAgesMatchRows();
life.reseed();
checkAgesMatchRows();

// Resize through the compiler and keep coordinates, ages and the counter.
life.install(defaults);
life.reset();
life.rows.fill(0n);
life.ages.fill(0);
for (const [x, y] of [[0, 0], [1, 0], [2, 0], [width - 2, height - 2], [width - 1, height - 2], [width - 1, height - 1]]) {
  life.toggle(x, y);
}
life.step();
assert.equal(life.ages[1], 2);
const beforeResize = life.ages.slice();
const smaller = compile(original
  .replace('grid-width = 40', 'grid-width = 3 + 5')
  .replace('grid-height = 24', 'grid-height = 6'));
assert.ok('ages' in smaller, JSON.stringify(smaller));
assert.equal(smaller.width, 8);
assert.equal(smaller.height, 6);
assert.throws(() => life.install({ ...smaller, width: 63 }), /invalid grid dimensions/);
assert.deepEqual(life.ages, beforeResize);
life.install(smaller);
assert.equal(life.generation, 1n);
assert.equal(life.ages.length, 48);
for (let y = 0; y < smaller.height; y++) {
  assert.deepEqual(life.ages.slice(y * 8, (y + 1) * 8), beforeResize.slice(y * width, y * width + 8));
}
checkAgesMatchRows();

const larger = compile(original.replace('grid-width = 40', 'grid-width = 62').replace('grid-height = 24', 'grid-height = 64'));
assert.ok('ages' in larger, JSON.stringify(larger));
life.install(larger);
assert.equal(life.generation, 1n);
assert.equal(life.ages[1], 2);
assert.ok(life.ages.slice(6 * larger.width).every((age) => age === 0));
for (let y = 0; y < smaller.height; y++) {
  assert.ok(life.ages.slice(y * larger.width + smaller.width, (y + 1) * larger.width).every((age) => age === 0));
}
checkAgesMatchRows();

// The widest board still has both wrap bits, including bit 63.
life.rows.fill(0n);
life.ages.fill(0);
life.toggle(larger.width - 1, 0);
life.toggle(0, 0);
life.toggle(1, 0);
life.step();
assert.equal(life.rows[0], 1n);
assert.equal(life.rows[1], 1n);
assert.equal(life.rows[larger.height - 1], 1n);
assert.equal(life.generation, 2n);

life.install({ ...defaults, width: 1, height: 1 });
life.reset();
assert.deepEqual([...life.ages], [1]);
life.step();
assert.deepEqual([...life.ages], [0]);
assert.equal(life.generation, 1n);

console.log('Cinch Wasm activated edits atomically and resized the grid while preserving overlapping cells, ages, and generation.');
