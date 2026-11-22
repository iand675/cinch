// Each packed row reserves two bits for its neighbours across the edges.
const maxWidth = 62;
const maxHeight = 64;
export const maxAge = 32;
const neighbourCounts = 9;
const neighbourMasks = 256;
const boardRepresentation = 3n;
const settingsSymbol = 0n;
const wordBytes = (maxHeight + 1) * BigUint64Array.BYTES_PER_ELEMENT;
const stateBytes = BigInt(wordBytes + maxWidth * maxHeight);

export interface LifeSettings {
  ages: number[];
  neighbours: number[];
  colors: number[];
  stepMs: number;
  wrapEdges: boolean;
  width: number;
  height: number;
}

type WordFunction = (...args: bigint[]) => bigint;
export interface BoardExports extends WebAssembly.Exports {
  memory: WebAssembly.Memory;
  life_initial_age: (age: number, count: number) => number;
  life_initial_neighbours: (mask: number) => number;
  life_initial_color: (age: number) => number;
  life_initial_step_ms: () => number;
  life_initial_wrap: () => number;
  life_initial_width: () => number;
  life_initial_height: () => number;
  life_neighbour_mask: (above: bigint, row: bigint, below: bigint, column: number) => number;
  cinch_alloc: WordFunction;
  cinch_evo_module_new: WordFunction;
  cinch_evo_state_seed: WordFunction;
  cinch_evo_state_payload: WordFunction;
  cinch_evo_current_version: WordFunction;
  cinch_evo_version_new: WordFunction;
  cinch_evo_update_begin: WordFunction;
  cinch_evo_add_replacement: WordFunction;
  cinch_evo_mark_complete: WordFunction;
  cinch_evo_activate: WordFunction;
  cinch_evo_version_try_drop: WordFunction;
}

function checked(value: bigint): bigint {
  if (value < 0n) throw new Error(`Cinch's runtime refused the update (${value}).`);
  return value;
}

function inRange(value: number, max: number): boolean {
  return Number.isInteger(value) && value >= 0 && value <= max;
}

function packRow(row: bigint, width: number, wrapEdges: boolean): bigint {
  const padded = row << 1n;
  if (!wrapEdges) return padded;
  return padded | (row >> BigInt(width - 1)) | ((row & 1n) << BigInt(width + 1));
}

export class Life {
  private readonly wasm: BoardExports;
  private readonly module: bigint;
  private readonly versions = new Map<bigint, LifeSettings>();

  static async load(boardUrl: string): Promise<Life> {
    const response = await fetch(boardUrl);
    if (!response.ok) throw new Error('The precompiled board could not be loaded.');
    const { instance } = await WebAssembly.instantiate(await response.arrayBuffer());
    return new Life(instance.exports as BoardExports);
  }

  constructor(wasm: BoardExports) {
    this.wasm = wasm;
    this.module = checked(wasm.cinch_evo_module_new(0n));
    // The runtime keeps this KIND_RAW block across updates. The maximum
    // capacity and generation slot are fixed; the active dimensions can vary.
    const payload = checked(wasm.cinch_alloc(stateBytes, 13n));
    checked(wasm.cinch_evo_state_seed(this.module, boardRepresentation, payload));
    this.install({
      ages: Array.from({ length: (maxAge + 1) * neighbourCounts }, (_, i) =>
        wasm.life_initial_age(Math.floor(i / neighbourCounts), i % neighbourCounts)),
      neighbours: Array.from({ length: neighbourMasks }, (_, mask) => wasm.life_initial_neighbours(mask)),
      colors: Array.from({ length: maxAge }, (_, i) => wasm.life_initial_color(i + 1)),
      stepMs: wasm.life_initial_step_ms(),
      wrapEdges: wasm.life_initial_wrap() === 1,
      width: wasm.life_initial_width(),
      height: wasm.life_initial_height(),
    });
    this.reset();
  }

  private get payload(): number {
    return Number(checked(this.wasm.cinch_evo_state_payload(this.module)));
  }

  private get words(): BigUint64Array {
    return new BigUint64Array(this.wasm.memory.buffer, this.payload, maxHeight + 1);
  }

  get rows(): BigUint64Array {
    return this.words.subarray(0, this.settings.height);
  }

  get generation(): bigint {
    return this.words[maxHeight];
  }

  get ages(): Uint8Array {
    const { width, height } = this.settings;
    return new Uint8Array(this.wasm.memory.buffer, this.payload + wordBytes, width * height);
  }

  get settings(): LifeSettings {
    const version = checked(this.wasm.cinch_evo_current_version(this.module));
    const settings = this.versions.get(version);
    if (!settings) throw new Error('The active Life settings are missing.');
    return settings;
  }

  install(settings: LifeSettings): number {
    const { ages, neighbours, colors, stepMs, wrapEdges, width, height } = settings;
    if (ages.length !== (maxAge + 1) * neighbourCounts || ages.some((age) => !inRange(age, maxAge))) {
      throw new Error('The compiler returned invalid cell ages.');
    }
    if (neighbours.length !== neighbourMasks || neighbours.some((n) => !inRange(n, neighbourCounts - 1))) {
      throw new Error('The compiler returned invalid neighbour counts.');
    }
    if (colors.length !== maxAge || colors.some((color) => !inRange(color, 0xffffff))) {
      throw new Error('The compiler returned an invalid palette.');
    }
    if (!inRange(stepMs, 0x7fffffff) || typeof wrapEdges !== 'boolean') {
      throw new Error('The compiler returned invalid board settings.');
    }
    if (!inRange(width, maxWidth) || width === 0 || !inRange(height, maxHeight) || height === 0) {
      throw new Error('The compiler returned invalid grid dimensions.');
    }

    const wasm = this.wasm;
    const previous = wasm.cinch_evo_current_version(this.module);
    const oldSettings = this.versions.get(previous);
    const resized = oldSettings && (oldSettings.width !== width || oldSettings.height !== height);
    const oldAges = resized ? this.ages.slice() : undefined;

    const version = checked(wasm.cinch_evo_version_new(this.module, boardRepresentation, 0n));
    const update = checked(wasm.cinch_evo_update_begin(this.module, version));
    checked(wasm.cinch_evo_add_replacement(update, settingsSymbol, version));
    checked(wasm.cinch_evo_mark_complete(update, 1n, 0n));
    this.versions.set(version, settings);
    let epoch;
    try {
      epoch = checked(wasm.cinch_evo_activate(update));
    } catch (error) {
      this.versions.delete(version);
      throw error;
    }
    if (oldSettings && oldAges) this.resize(oldSettings, oldAges);
    if (previous >= 0n && wasm.cinch_evo_version_try_drop(this.module, previous) === 1n) {
      this.versions.delete(previous);
    }
    return Number(epoch);
  }

  private resize(previous: LifeSettings, oldAges: Uint8Array): void {
    const { width, height } = this.settings;
    const rows = this.rows;
    const ages = this.ages;
    const columnsToKeep = Math.min(previous.width, width);
    const rowsToKeep = Math.min(previous.height, height);
    const mask = (1n << BigInt(columnsToKeep)) - 1n;

    ages.fill(0);
    rows.fill(0n, rowsToKeep);
    for (let y = 0; y < rowsToKeep; y++) {
      rows[y] &= mask;
      const start = y * previous.width;
      ages.set(oldAges.subarray(start, start + columnsToKeep), y * width);
    }
  }

  step(): void {
    const settings = this.settings;
    const { width, height, wrapEdges } = settings;
    const words = this.words;
    const rows = words.subarray(0, height);
    const ages = this.ages;
    const padded = rows.map((row) => packRow(row, width, wrapEdges));
    for (let y = 0; y < height; y++) {
      const above = y > 0 ? padded[y - 1] : wrapEdges ? padded[height - 1] : 0n;
      const below = y + 1 < height ? padded[y + 1] : wrapEdges ? padded[0] : 0n;
      let next = 0n;
      for (let x = 0; x < width; x++) {
        const mask = this.wasm.life_neighbour_mask(above, padded[y], below, x);
        const i = y * width + x;
        const age = settings.ages[ages[i] * neighbourCounts + settings.neighbours[mask]];
        ages[i] = age;
        if (age > 0) next |= 1n << BigInt(x);
      }
      rows[y] = next;
    }
    words[maxHeight]++;
  }

  toggle(x: number, y: number): void {
    const rows = this.rows;
    const bit = 1n << BigInt(x);
    rows[y] ^= bit;
    this.ages[y * this.settings.width + x] = (rows[y] & bit) === 0n ? 0 : 1;
  }

  reseed(): void {
    const { width, height } = this.settings;
    const rows = this.rows;
    const ages = this.ages;
    rows.fill(0n);
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const age = Math.random() < 0.3 ? 1 : 0;
        ages[y * width + x] = age;
        rows[y] |= BigInt(age) << BigInt(x);
      }
    }
  }

  reset(): void {
    const gun = [
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
    const { width, height } = this.settings;
    const fitsGun = width > gun[0].length && height > gun.length;
    const pattern = fitsGun ? gun : width >= 3 && height >= 3 ? ['.#.', '..#', '###'] : ['#'];
    const left = fitsGun ? 1 : Math.floor((width - pattern[0].length) / 2);
    const top = fitsGun ? 1 : Math.floor((height - pattern.length) / 2);
    this.words.fill(0n);
    const rows = this.rows;
    const ages = this.ages;
    ages.fill(0);
    for (const [y, row] of pattern.entries()) {
      for (const [x, cell] of [...row].entries()) {
        if (cell === '#') {
          rows[y + top] |= 1n << BigInt(x + left);
          ages[(y + top) * width + x + left] = 1;
        }
      }
    }
  }
}
