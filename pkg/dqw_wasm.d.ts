/* tslint:disable */
/* eslint-disable */
/**
*/
export function initialize(): void;
/**
* @param {string} monsters
* @param {any} options
* @returns {any}
*/
export function return_all_combis3_csv(monsters: string, options: any): any;
/**
* @param {string} monsters
* @param {any} options
* @returns {any}
*/
export function return_all_combis2_csv(monsters: string, options: any): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly initialize: () => void;
  readonly return_all_combis3_csv: (a: number, b: number, c: number) => number;
  readonly return_all_combis2_csv: (a: number, b: number, c: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
