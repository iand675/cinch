import { ConsoleStdout, File, OpenFile, WASI } from '@bjorn3/browser_wasi_shim';
import type { LifeSettings } from './life';
import compilerUrl from '../data/examples/life-compiler.wasm?url';

interface CompilerExports extends WebAssembly.Exports {
  memory: WebAssembly.Memory;
  life_input: () => number;
  life_compile: (length: number) => number;
  life_output: () => number;
}

export type CompileRequest = { id: number; source: string };
export type CompileReply = { id: number } & ({ settings: LifeSettings } | { error: string; restart?: boolean });

const compiler = (async () => {
  const response = await fetch(compilerUrl);
  if (!response.ok) throw new Error('The Cinch compiler could not be loaded.');
  const wasi = new WASI([], [], [
    new OpenFile(new File([])),
    ConsoleStdout.lineBuffered(() => {}),
    ConsoleStdout.lineBuffered(() => {}),
  ]);
  const { instance } = await WebAssembly.instantiate(await response.arrayBuffer(), {
    wasi_snapshot_preview1: wasi.wasiImport,
  });
  const exports = instance.exports as CompilerExports;
  wasi.initialize({ exports });
  return exports;
})();

self.addEventListener('message', async ({ data }: MessageEvent<CompileRequest>) => {
  const { id, source } = data;
  let reply: CompileReply;
  try {
    const wasm = await compiler;
    const input = new TextEncoder().encode(source);
    if (input.length <= 4096) new Uint8Array(wasm.memory.buffer, wasm.life_input(), input.length).set(input);
    const length = wasm.life_compile(input.length);
    const output = new Uint8Array(wasm.memory.buffer, wasm.life_output(), length);
    const result: LifeSettings | { error: string } = JSON.parse(new TextDecoder().decode(output));
    reply = 'error' in result ? { id, error: result.error } : { id, settings: result };
  } catch {
    reply = { id, error: 'The Cinch compiler stopped. Edit the code to retry.', restart: true };
  }
  self.postMessage(reply);
});
