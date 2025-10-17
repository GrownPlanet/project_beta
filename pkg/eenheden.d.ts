/* tslint:disable */
/* eslint-disable */
export function get_time_formats(): string[];
export function get_distance_formats(): string[];
export function convert_seconds(t: number, to: string): number | undefined;
export function convert_meters(x: number, to: string): number | undefined;
export function convert_speed(v: number, dist: string, time: string): number | undefined;
export function convert_acceleration(a: number, dist: string, time: string): number | undefined;
export function find_best(v: number, mode: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_time_formats: () => [number, number];
  readonly get_distance_formats: () => [number, number];
  readonly convert_seconds: (a: number, b: number, c: number) => [number, number];
  readonly convert_meters: (a: number, b: number, c: number) => [number, number];
  readonly convert_speed: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly convert_acceleration: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly find_best: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
