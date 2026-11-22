export const width = 40;
export const height = 24;
const boardRepresentation = 1n;
const ruleSymbol = 0n;
const stateBytes = BigInt((height + 1) * BigUint64Array.BYTES_PER_ELEMENT);

type WordFunction = (...args: bigint[]) => bigint;
export interface BoardExports extends WebAssembly.Exports {
  memory: WebAssembly.Memory;
  life_initial_rule: () => number;
  life_step_row: (rules: number, above: bigint, row: bigint, below: bigint) => bigint;
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

export class Life {
  private readonly wasm: BoardExports;
  private readonly module: bigint;
  private readonly rules = new Map<bigint, number>();
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
    // A KIND_RAW block holds 24 packed rows and the generation counter.
    // The runtime keeps the same payload across updates of this representation.
    const payload = checked(wasm.cinch_alloc(stateBytes, 13n));
    checked(wasm.cinch_evo_state_seed(this.module, boardRepresentation, payload));
    this.install(wasm.life_initial_rule());
    this.reset();
  }

  get state(): BigUint64Array {
    const pointer = checked(this.wasm.cinch_evo_state_payload(this.module));
    return new BigUint64Array(this.wasm.memory.buffer, Number(pointer), height + 1);
  }

  install(table: number): number {
    if (!Number.isInteger(table) || table < 0 || table >= 1 << 18) {
      throw new Error('The compiler returned an invalid rule.');
    }
    const wasm = this.wasm;
    const previous = wasm.cinch_evo_current_version(this.module);
    const version = checked(wasm.cinch_evo_version_new(this.module, boardRepresentation, 0n));
    const update = checked(wasm.cinch_evo_update_begin(this.module, version));
    checked(wasm.cinch_evo_add_replacement(update, ruleSymbol, version));
    checked(wasm.cinch_evo_mark_complete(update, 1n, 0n));
    this.rules.set(version, table);
    let epoch;
    try {
      epoch = checked(wasm.cinch_evo_activate(update));
    } catch (error) {
      this.rules.delete(version);
      throw error;
    }
    if (previous >= 0n && wasm.cinch_evo_version_try_drop(this.module, previous) === 1n) {
      this.rules.delete(previous);
    }
    return Number(epoch);
  }

  step(): void {
    const wasm = this.wasm;
    // The main thread steps whole generations synchronously. This read pins
    // the dispatch choice for the generation before the next message can
    // activate a version.
    const version = checked(wasm.cinch_evo_current_version(this.module));
    const table = this.rules.get(version);
    if (table === undefined) throw new Error('The active rule is missing.');
    const state = this.state;
    for (let y = 0; y < height; y++) {
      const above = state[y - 1] ?? 0n;
      const below = y + 1 < height ? state[y + 1] : 0n;
      this.next[y] = wasm.life_step_row(table, above, state[y], below);
    }
    state.set(this.next);
    state[height]++;
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
    state.fill(0n);
    for (const [y, row] of gun.entries()) {
      for (const [x, cell] of [...row].entries()) {
        if (cell === '#') state[y + 1] |= 1n << BigInt(x + 1);
      }
    }
  }
}
