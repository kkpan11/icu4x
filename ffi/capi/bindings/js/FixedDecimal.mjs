// generated by diplomat-tool
import { FixedDecimalLimitError } from "./FixedDecimalLimitError.mjs"
import { FixedDecimalParseError } from "./FixedDecimalParseError.mjs"
import { FixedDecimalRoundingIncrement } from "./FixedDecimalRoundingIncrement.mjs"
import { FixedDecimalRoundingMode } from "./FixedDecimalRoundingMode.mjs"
import { FixedDecimalSign } from "./FixedDecimalSign.mjs"
import { FixedDecimalSignDisplay } from "./FixedDecimalSignDisplay.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
*/

const FixedDecimal_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.ICU4XFixedDecimal_destroy(ptr);
});
export class FixedDecimal {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        FixedDecimal_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static fromInteger(v) {
        const result = wasm.ICU4XFixedDecimal_create_from_i32(v);
    
        try {
    
            return new FixedDecimal(result, []);
        } finally {
        
        }
    }

    static fromBigInt(v) {
        const result = wasm.ICU4XFixedDecimal_create_from_i64(v);
    
        try {
    
            return new FixedDecimal(result, []);
        } finally {
        
        }
    }

    static fromNumberWithLowerMagnitude(f, magnitude) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(diplomat_receive_buffer, f, magnitude);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = FixedDecimalLimitError[Array.from(FixedDecimalLimitError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('FixedDecimalLimitError: ' + cause.value, { cause });
            }
            return new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static fromNumberWithSignificantDigits(f, digits) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ICU4XFixedDecimal_create_from_f64_with_significant_digits(diplomat_receive_buffer, f, digits);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = FixedDecimalLimitError[Array.from(FixedDecimalLimitError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('FixedDecimalLimitError: ' + cause.value, { cause });
            }
            return new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static fromNumberWithFloatingPrecision(f) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ICU4XFixedDecimal_create_from_f64_with_floating_precision(diplomat_receive_buffer, f);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = FixedDecimalLimitError[Array.from(FixedDecimalLimitError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('FixedDecimalLimitError: ' + cause.value, { cause });
            }
            return new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static fromString(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.str8(wasm, v);
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ICU4XFixedDecimal_create_from_string(diplomat_receive_buffer, vSlice.ptr, vSlice.size);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = FixedDecimalParseError[Array.from(FixedDecimalParseError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('FixedDecimalParseError: ' + cause.value, { cause });
            }
            return new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            vSlice.free();
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    digitAt(magnitude) {
        const result = wasm.ICU4XFixedDecimal_digit_at(this.ffiValue, magnitude);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get magnitudeStart() {
        const result = wasm.ICU4XFixedDecimal_magnitude_start(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get magnitudeEnd() {
        const result = wasm.ICU4XFixedDecimal_magnitude_end(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get nonzeroMagnitudeStart() {
        const result = wasm.ICU4XFixedDecimal_nonzero_magnitude_start(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get nonzeroMagnitudeEnd() {
        const result = wasm.ICU4XFixedDecimal_nonzero_magnitude_end(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get isZero() {
        const result = wasm.ICU4XFixedDecimal_is_zero(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    multiplyPow10(power) {
        wasm.ICU4XFixedDecimal_multiply_pow10(this.ffiValue, power);
    
        try {
    
        } finally {
        
        }
    }

    get sign() {
        const result = wasm.ICU4XFixedDecimal_sign(this.ffiValue);
    
        try {
    
            return FixedDecimalSign[Array.from(FixedDecimalSign.values.keys())[result]];
        } finally {
        
        }
    }

    set sign(sign) {
        wasm.ICU4XFixedDecimal_set_sign(this.ffiValue, sign.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    applySignDisplay(signDisplay) {
        wasm.ICU4XFixedDecimal_apply_sign_display(this.ffiValue, signDisplay.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    trimStart() {
        wasm.ICU4XFixedDecimal_trim_start(this.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    trimEnd() {
        wasm.ICU4XFixedDecimal_trim_end(this.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    padStart(position) {
        wasm.ICU4XFixedDecimal_pad_start(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    padEnd(position) {
        wasm.ICU4XFixedDecimal_pad_end(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    setMaxPosition(position) {
        wasm.ICU4XFixedDecimal_set_max_position(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    round(position) {
        wasm.ICU4XFixedDecimal_round(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    ceil(position) {
        wasm.ICU4XFixedDecimal_ceil(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    expand(position) {
        wasm.ICU4XFixedDecimal_expand(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    floor(position) {
        wasm.ICU4XFixedDecimal_floor(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    trunc(position) {
        wasm.ICU4XFixedDecimal_trunc(this.ffiValue, position);
    
        try {
    
        } finally {
        
        }
    }

    roundWithMode(position, mode) {
        wasm.ICU4XFixedDecimal_round_with_mode(this.ffiValue, position, mode.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    roundWithModeAndIncrement(position, mode, increment) {
        wasm.ICU4XFixedDecimal_round_with_mode_and_increment(this.ffiValue, position, mode.ffiValue, increment.ffiValue);
    
        try {
    
        } finally {
        
        }
    }

    concatenateEnd(other) {
        const result = wasm.ICU4XFixedDecimal_concatenate_end(this.ffiValue, other.ffiValue);
    
        try {
    
            return result == 1;
        } finally {
        
        }
    }

    toString() {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.ICU4XFixedDecimal_to_string(this.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    

}