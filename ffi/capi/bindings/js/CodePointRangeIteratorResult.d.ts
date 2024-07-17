// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** Result of a single iteration of [`CodePointRangeIterator`].
*Logically can be considered to be an `Option<RangeInclusive<u32>>`,
*
*`start` and `end` represent an inclusive range of code points [start, end],
*and `done` will be true if the iterator has already finished. The last contentful
*iteration will NOT produce a range done=true, in other words `start` and `end` are useful
*values if and only if `done=false`.
*/
export class CodePointRangeIteratorResult {
    get start() : number;
    
    get end() : number;
    
    get done() : boolean;
    

    

}