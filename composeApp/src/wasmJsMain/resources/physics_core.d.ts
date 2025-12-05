/* tslint:disable */
/* eslint-disable */

export function wasm_get_info(): string;

export function wasm_init(width: number, height: number): boolean;

export function wasm_render(): void;

export function wasm_resize(width: number, height: number): void;

export function wasm_shutdown(): void;

export function wasm_update(delta_time: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly physics_core_get_info: () => number;
  readonly physics_core_free_string: (a: number) => void;
  readonly wgpu_init: (a: number, b: number) => number;
  readonly wgpu_update: (a: number) => void;
  readonly wgpu_resize: (a: number, b: number) => void;
  readonly wasm_get_info: () => [number, number];
  readonly wasm_init: (a: number, b: number) => number;
  readonly wasm_update: (a: number) => void;
  readonly wasm_render: () => void;
  readonly wasm_resize: (a: number, b: number) => void;
  readonly wasm_shutdown: () => void;
  readonly wgpu_shutdown: () => void;
  readonly wgpu_render: () => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
