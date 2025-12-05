/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const physics_core_get_info: () => number;
export const physics_core_free_string: (a: number) => void;
export const wgpu_init: (a: number, b: number) => number;
export const wgpu_update: (a: number) => void;
export const wgpu_resize: (a: number, b: number) => void;
export const wasm_get_info: () => [number, number];
export const wasm_init: (a: number, b: number) => number;
export const wasm_update: (a: number) => void;
export const wasm_render: () => void;
export const wasm_resize: (a: number, b: number) => void;
export const wasm_shutdown: () => void;
export const wgpu_shutdown: () => void;
export const wgpu_render: () => void;
export const __wbindgen_externrefs: WebAssembly.Table;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
