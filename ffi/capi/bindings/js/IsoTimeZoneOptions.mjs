// generated by diplomat-tool
import { IsoTimeZoneFormat } from "./IsoTimeZoneFormat.mjs"
import { IsoTimeZoneMinuteDisplay } from "./IsoTimeZoneMinuteDisplay.mjs"
import { IsoTimeZoneSecondDisplay } from "./IsoTimeZoneSecondDisplay.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

export class IsoTimeZoneOptions {
    #format;
    get format()  {
        return this.#format;
    }
    set format(value) {
        this.#format = value;
    }
    #minutes;
    get minutes()  {
        return this.#minutes;
    }
    set minutes(value) {
        this.#minutes = value;
    }
    #seconds;
    get seconds()  {
        return this.#seconds;
    }
    set seconds(value) {
        this.#seconds = value;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        slice_cleanup_callbacks,
        appendArrayMap
    ) {
        return [this.#format.ffiValue, this.#minutes.ffiValue, this.#seconds.ffiValue]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    _fromFFI(ptr) {
        const formatDeref = diplomatRuntime.enumDiscriminant(wasm, ptr);
        this.#format = IsoTimeZoneFormat[Array.from(IsoTimeZoneFormat.values.keys())[formatDeref]];
        const minutesDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 4);
        this.#minutes = IsoTimeZoneMinuteDisplay[Array.from(IsoTimeZoneMinuteDisplay.values.keys())[minutesDeref]];
        const secondsDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 8);
        this.#seconds = IsoTimeZoneSecondDisplay[Array.from(IsoTimeZoneSecondDisplay.values.keys())[secondsDeref]];

        return this;
    }
    

}