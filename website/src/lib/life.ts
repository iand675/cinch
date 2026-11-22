export const width = 40;
export const height = 24;
export const maxAge = 32;
const neighbourCounts = 9;
const neighbourMasks = 256;
const boardRepresentation = 2n;
const settingsSymbol = 0n;
const rowBytes = (height + 1) * BigUint64Array.BYTES_PER_ELEMENT;
const stateBytes = BigInt(rowBytes + width * height);

export interface LifeSettings {
  ages: number[];
  neighbours: number[];
  colors: number[];
  stepMs: number;
  wrapEdges: boolean;
}

type WordFunction = (...args: bigint[]) => bigint;
export interface BoardExports extends WebAssembly.Exports {
  memory: WebAssembly.Memory;
  life_initial_age: (age: number, count: number) => number;
  life_initial_neighbours: (mask: number) => number;
  life_initial_color: (age: number) => number;
  life_initial_step_ms: () => number;
  life_initial_wrap: () => number;
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

function packRow(row: bigint, wrapEdges: boolean): bigint {
  const padded = row << 1n;
  if (!wrapEdges) return padded;
  return padded | (row >> BigInt(width - 1)) | ((row & 1n) << BigInt(width + 1));
}

export class Life {
  private readonly wasm: BoardExports;
  private readonly module: bigint;
  private readonly versions = new Map<bigint, LifeSettings>();
  private readonly next = new BigUint64Array(height);

  static async load(boardUrl: string): Promise<Life> {
    const response = await fetch(boardUrl);
    if (!response.ok) throw new Error('The precompiled board could not be loaded.');
    const { instance } = await WebAssembly.instantiate(await response.arrayBuffer());
    return new Life(instance.exports as BoardExports);
  }

  constructor(wasm: BoardExports) {
    this.wasm = wasm;
    this.module = checked(wasm.cinch_evo_module_new(0n));
    // The runtime keeps this KIND_RAW block across updates of the same
    // representation. It holds packed rows, a generation counter, and ages.
    const payload = checked(wasm.cinch_alloc(stateBytes, 13n));
    checked(wasm.cinch_evo_state_seed(this.module, boardRepresentation, payload));
    this.install({
      ages: Array.from({ length: (maxAge + 1) * neighbourCounts }, (_, i) =>
        wasm.life_initial_age(Math.floor(i / neighbourCounts), i % neighbourCounts)),
      neighbours: Array.from({ length: neighbourMasks }, (_, mask) => wasm.life_initial_neighbours(mask)),
      colors: Array.from({ length: maxAge }, (_, i) => wasm.life_initial_color(i + 1)),
      stepMs: wasm.life_initial_step_ms(),
      wrapEdges: wasm.life_initial_wrap() === 1,
    });
    this.reset();
  }

  private get payload(): number {
    return Number(checked(this.wasm.cinch_evo_state_payload(this.module)));
  }

  get state(): BigUint64Array {
    return new BigUint64Array(this.wasm.memory.buffer, this.payload, height + 1);
  }

  get ages(): Uint8Array {
    return new Uint8Array(this.wasm.memory.buffer, this.payload + rowBytes, width * height);
  }

  get settings(): LifeSettings {
    const version = checked(this.wasm.cinch_evo_current_version(this.module));
    const settings = this.versions.get(version);
    if (!settings) throw new Error('The active Life settings are missing.');
    return settings;
  }

  install(settings: LifeSettings): number {
    const { ages, neighbours, colors, stepMs, wrapEdges } = settings;
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
    const wasm = this.wasm;
    const previous = wasm.cinch_evo_current_version(this.module);
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
    if (previous >= 0n && wasm.cinch_evo_version_try_drop(this.module, previous) === 1n) {
      this.versions.delete(previous);
    }
    return Number(epoch);
  }

  step(): void {
    const settings = this.settings;
    const state = this.state;
    const ages = this.ages;
    const padded = state.subarray(0, height).map((row) => packRow(row, settings.wrapEdges));
    for (let y = 0; y < height; y++) {
      const above = y > 0 ? padded[y - 1] : settings.wrapEdges ? padded[height - 1] : 0n;
      const below = y + 1 < height ? padded[y + 1] : settings.wrapEdges ? padded[0] : 0n;
      let next = 0n;
      for (let x = 0; x < width; x++) {
        const mask = this.wasm.life_neighbour_mask(above, padded[y], below, x);
        const i = y * width + x;
        const age = settings.ages[ages[i] * neighbourCounts + settings.neighbours[mask]];
        ages[i] = age;
        if (age > 0) next |= 1n << BigInt(x);
      }
      this.next[y] = next;
    }
    state.set(this.next);
    state[height]++;
  }

  toggle(x: number, y: number): void {
    const state = this.state;
    const bit = 1n << BigInt(x);
    state[y] ^= bit;
    this.ages[y * width + x] = (state[y] & bit) === 0n ? 0 : 1;
  }

  reseed(): void {
    const state = this.state;
    const ages = this.ages;
    state.fill(0n);
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const age = Math.random() < 0.3 ? 1 : 0;
        ages[y * width + x] = age;
        state[y] |= BigInt(age) << BigInt(x);
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
    const state = this.state;
    const ages = this.ages;
    state.fill(0n);
    ages.fill(0);
    for (const [y, row] of gun.entries()) {
      for (const [x, cell] of [...row].entries()) {
        if (cell === '#') {
          state[y + 1] |= 1n << BigInt(x + 1);
          ages[(y + 1) * width + x + 1] = 1;
        }
      }
    }
  }
}
