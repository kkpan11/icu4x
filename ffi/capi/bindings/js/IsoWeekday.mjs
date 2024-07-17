// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
export class IsoWeekday {
    #value = undefined;

    static values = new Map([
        ["Monday", 1],
        ["Tuesday", 2],
        ["Wednesday", 3],
        ["Thursday", 4],
        ["Friday", 5],
        ["Saturday", 6],
        ["Sunday", 7]
    ]);
    constructor(value) {
        if (value instanceof IsoWeekday) {
            this.#value = value.value;
            return;
        }

        if (IsoWeekday.values.has(value)) {
            this.#value = value;
            return;
        }

        throw TypeError(value + " is not a IsoWeekday and does not correspond to any of its enumerator values.");
    }

    get value() {
        return this.#value;
    }

    get ffiValue() {
        return IsoWeekday.values.get(this.#value);
    }

    static Monday = new IsoWeekday("Monday");

    static Tuesday = new IsoWeekday("Tuesday");

    static Wednesday = new IsoWeekday("Wednesday");

    static Thursday = new IsoWeekday("Thursday");

    static Friday = new IsoWeekday("Friday");

    static Saturday = new IsoWeekday("Saturday");

    static Sunday = new IsoWeekday("Sunday");


    

}