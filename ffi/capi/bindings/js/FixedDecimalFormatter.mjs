// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { FixedDecimal } from "./FixedDecimal.mjs"
import { FixedDecimalGroupingStrategy } from "./FixedDecimalGroupingStrategy.mjs"
import { Locale } from "./Locale.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X Fixed Decimal Format object, capable of formatting a [`FixedDecimal`] as a string.
*
*See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
*/

const FixedDecimalFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.ICU4XFixedDecimalFormatter_destroy(ptr);
});
export class FixedDecimalFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        FixedDecimalFormatter_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static createWithGroupingStrategy(provider, locale, groupingStrategy) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ICU4XFixedDecimalFormatter_create_with_grouping_strategy(diplomat_receive_buffer, provider.ffiValue, locale.ffiValue, groupingStrategy.ffiValue);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = DataError[Array.from(DataError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('DataError: ' + cause.value, { cause });
            }
            return new FixedDecimalFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    format(value) {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.ICU4XFixedDecimalFormatter_format(this.ffiValue, value.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    

}