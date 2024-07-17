// generated by diplomat-tool
import { Calendar } from "./Calendar.mjs"
import { CalendarError } from "./CalendarError.mjs"
import { Date } from "./Date.mjs"
import { IsoWeekday } from "./IsoWeekday.mjs"
import { WeekCalculator } from "./WeekCalculator.mjs"
import { WeekOf } from "./WeekOf.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X Date object capable of containing a ISO-8601 date
*
*See the [Rust documentation for `Date`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html) for more information.
*/

const IsoDate_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.ICU4XIsoDate_destroy(ptr);
});
export class IsoDate {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        IsoDate_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static create(year, month, day) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.ICU4XIsoDate_create(diplomat_receive_buffer, year, month, day);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = CalendarError[Array.from(CalendarError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('CalendarError: ' + cause.value, { cause });
            }
            return new IsoDate(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static createForUnixEpoch() {
        const result = wasm.ICU4XIsoDate_create_for_unix_epoch();
    
        try {
    
            return new IsoDate(result, []);
        } finally {
        
        }
    }

    toCalendar(calendar) {
        const result = wasm.ICU4XIsoDate_to_calendar(this.ffiValue, calendar.ffiValue);
    
        try {
    
            return new Date(result, []);
        } finally {
        
        }
    }

    toAny() {
        const result = wasm.ICU4XIsoDate_to_any(this.ffiValue);
    
        try {
    
            return new Date(result, []);
        } finally {
        
        }
    }

    get dayOfYear() {
        const result = wasm.ICU4XIsoDate_day_of_year(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get dayOfMonth() {
        const result = wasm.ICU4XIsoDate_day_of_month(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get dayOfWeek() {
        const result = wasm.ICU4XIsoDate_day_of_week(this.ffiValue);
    
        try {
    
            return (() => {for (let i of IsoWeekday.values) { if(i[1] === result) return IsoWeekday[i[0]]; } return null;})();
        } finally {
        
        }
    }

    weekOfMonth(firstWeekday) {
        const result = wasm.ICU4XIsoDate_week_of_month(this.ffiValue, firstWeekday.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    weekOfYear(calculator) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
        const result = wasm.ICU4XIsoDate_week_of_year(diplomat_receive_buffer, this.ffiValue, calculator.ffiValue);
    
        try {
    
            return new WeekOf(diplomat_receive_buffer);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
        
        }
    }

    get month() {
        const result = wasm.ICU4XIsoDate_month(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get year() {
        const result = wasm.ICU4XIsoDate_year(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get isInLeapYear() {
        const result = wasm.ICU4XIsoDate_is_in_leap_year(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get monthsInYear() {
        const result = wasm.ICU4XIsoDate_months_in_year(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get daysInMonth() {
        const result = wasm.ICU4XIsoDate_days_in_month(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get daysInYear() {
        const result = wasm.ICU4XIsoDate_days_in_year(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    

}