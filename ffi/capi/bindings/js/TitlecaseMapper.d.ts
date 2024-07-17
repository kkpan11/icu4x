// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { Locale } from "./Locale"
import type { TitlecaseOptions } from "./TitlecaseOptions"
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `TitlecaseMapper`](https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html) for more information.
*/
export class TitlecaseMapper {
    

    get ffiValue(): pointer;


    static create(provider: DataProvider): TitlecaseMapper;

    titlecaseSegment(s: string, locale: Locale, options: TitlecaseOptions): string;

    

}