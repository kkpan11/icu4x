// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** What additional data is required to load when performing fallback.
*
*See the [Rust documentation for `LocaleFallbackSupplement`](https://docs.rs/icu/latest/icu/locale/fallback/enum.LocaleFallbackSupplement.html) for more information.
*/
export class LocaleFallbackSupplement {
    constructor(value : LocaleFallbackSupplement | string);

    get value() : string;

    get ffiValue() : number;

    static None : LocaleFallbackSupplement;

    static Collation : LocaleFallbackSupplement;


    

}