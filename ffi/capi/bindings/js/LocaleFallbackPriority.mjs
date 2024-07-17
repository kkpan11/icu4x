// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** Priority mode for the ICU4X fallback algorithm.
*
*See the [Rust documentation for `LocaleFallbackPriority`](https://docs.rs/icu/latest/icu/locale/fallback/enum.LocaleFallbackPriority.html) for more information.
*/
export class LocaleFallbackPriority {
    #value = undefined;

    static values = new Map([
        ["Language", 0],
        ["Region", 1],
        ["Collation", 2]
    ]);
    constructor(value) {
        if (value instanceof LocaleFallbackPriority) {
            this.#value = value.value;
            return;
        }

        if (LocaleFallbackPriority.values.has(value)) {
            this.#value = value;
            return;
        }

        throw TypeError(value + " is not a LocaleFallbackPriority and does not correspond to any of its enumerator values.");
    }

    get value() {
        return this.#value;
    }

    get ffiValue() {
        return LocaleFallbackPriority.values.get(this.#value);
    }

    static Language = new LocaleFallbackPriority("Language");

    static Region = new LocaleFallbackPriority("Region");

    static Collation = new LocaleFallbackPriority("Collation");


    

}