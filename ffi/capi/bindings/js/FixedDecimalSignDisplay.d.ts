// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** ECMA-402 compatible sign display preference.
*
*See the [Rust documentation for `SignDisplay`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.SignDisplay.html) for more information.
*/
export class FixedDecimalSignDisplay {
    constructor(value : FixedDecimalSignDisplay | string);

    get value() : string;

    get ffiValue() : number;

    static Auto : FixedDecimalSignDisplay;

    static Never : FixedDecimalSignDisplay;

    static Always : FixedDecimalSignDisplay;

    static ExceptZero : FixedDecimalSignDisplay;

    static Negative : FixedDecimalSignDisplay;


    

}