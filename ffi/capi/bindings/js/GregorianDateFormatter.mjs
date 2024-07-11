import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DateLength_js_to_rust, DateLength_rust_to_js } from "./DateLength.mjs"
import { Error_js_to_rust, Error_rust_to_js } from "./Error.mjs"

const GregorianDateFormatter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XGregorianDateFormatter_destroy(underlying);
});

export class GregorianDateFormatter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      GregorianDateFormatter_box_destroy_registry.register(this, underlying);
    }
  }

  static create_with_length(arg_provider, arg_locale, arg_length) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XGregorianDateFormatter_create_with_length(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, DateLength_js_to_rust[arg_length]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new GregorianDateFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = Error_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  format_iso_date(arg_value) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XGregorianDateFormatter_format_iso_date(this.underlying, arg_value.underlying, write);
    });
  }

  format_iso_datetime(arg_value) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XGregorianDateFormatter_format_iso_datetime(this.underlying, arg_value.underlying, write);
    });
  }
}