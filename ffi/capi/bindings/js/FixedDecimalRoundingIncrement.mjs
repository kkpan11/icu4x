// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** Increment used in a rounding operation.
*
*See the [Rust documentation for `RoundingIncrement`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.RoundingIncrement.html) for more information.
*/
export class FixedDecimalRoundingIncrement {
    #value = undefined;

    static values = new Map([
        ["MultiplesOf1", 0],
        ["MultiplesOf2", 1],
        ["MultiplesOf5", 2],
        ["MultiplesOf25", 3]
    ]);
    constructor(value) {
        if (value instanceof FixedDecimalRoundingIncrement) {
            this.#value = value.value;
            return;
        }

        if (FixedDecimalRoundingIncrement.values.has(value)) {
            this.#value = value;
            return;
        }

        throw TypeError(value + " is not a FixedDecimalRoundingIncrement and does not correspond to any of its enumerator values.");
    }

    get value() {
        return this.#value;
    }

    get ffiValue() {
        return FixedDecimalRoundingIncrement.values.get(this.#value);
    }

    static MultiplesOf1 = new FixedDecimalRoundingIncrement("MultiplesOf1");

    static MultiplesOf2 = new FixedDecimalRoundingIncrement("MultiplesOf2");

    static MultiplesOf5 = new FixedDecimalRoundingIncrement("MultiplesOf5");

    static MultiplesOf25 = new FixedDecimalRoundingIncrement("MultiplesOf25");


    

}