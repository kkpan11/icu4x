import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export const ICU4XError_js_to_rust = {
  "UnknownError": 0,
  "WriteableError": 1,
  "OutOfBoundsError": 2,
  "DataMissingDataKeyError": 256,
  "DataMissingVariantError": 257,
  "DataMissingLocaleError": 258,
  "DataNeedsVariantError": 259,
  "DataNeedsLocaleError": 260,
  "DataExtraneousLocaleError": 261,
  "DataFilteredResourceError": 262,
  "DataMismatchedTypeError": 263,
  "DataMissingPayloadError": 264,
  "DataInvalidStateError": 265,
  "DataCustomError": 266,
  "DataIoError": 267,
  "DataUnavailableBufferFormatError": 268,
  "LocaleUndefinedSubtagError": 512,
  "LocaleParserError": 513,
  "DataStructValidityError": 768,
  "PropertyUnknownScriptIdError": 1024,
  "PropertyUnknownGeneralCategoryGroupError": 1025,
  "DecimalLimitError": 1280,
  "DecimalSyntaxError": 1281,
  "PluralParserError": 1536,
  "DateTimeParseError": 1792,
  "DateTimeOverflowError": 1793,
  "DateTimeUnderflowError": 1794,
  "DateTimeInvalidTimeZoneOffsetError": 1795,
  "DateTimeOutOfRangeError": 1796,
  "DateTimeMissingInputError": 1797,
  "DateTimeFormatPatternError": 2048,
  "DateTimeFormatMissingInputFieldError": 2049,
  "DateTimeFormatSkeletonError": 2050,
  "DateTimeFormatUnsupportedFieldError": 2051,
  "DateTimeFormatUnsupportedOptionsError": 2052,
  "DateTimeFormatMissingWeekdaySymbolError": 2053,
  "DateTimeFormatMissingMonthSymbolError": 2054,
  "DateTimeFormatFixedDecimalError": 2055,
  "DateTimeFormatMismatchedAnyCalendarError": 2056,
  "DateTimeFormatMismatchedCalendarLocaleError": 2057,
};

export const ICU4XError_rust_to_js = {
  0: "UnknownError",
  1: "WriteableError",
  2: "OutOfBoundsError",
  256: "DataMissingDataKeyError",
  257: "DataMissingVariantError",
  258: "DataMissingLocaleError",
  259: "DataNeedsVariantError",
  260: "DataNeedsLocaleError",
  261: "DataExtraneousLocaleError",
  262: "DataFilteredResourceError",
  263: "DataMismatchedTypeError",
  264: "DataMissingPayloadError",
  265: "DataInvalidStateError",
  266: "DataCustomError",
  267: "DataIoError",
  268: "DataUnavailableBufferFormatError",
  512: "LocaleUndefinedSubtagError",
  513: "LocaleParserError",
  768: "DataStructValidityError",
  1024: "PropertyUnknownScriptIdError",
  1025: "PropertyUnknownGeneralCategoryGroupError",
  1280: "DecimalLimitError",
  1281: "DecimalSyntaxError",
  1536: "PluralParserError",
  1792: "DateTimeParseError",
  1793: "DateTimeOverflowError",
  1794: "DateTimeUnderflowError",
  1795: "DateTimeInvalidTimeZoneOffsetError",
  1796: "DateTimeOutOfRangeError",
  1797: "DateTimeMissingInputError",
  2048: "DateTimeFormatPatternError",
  2049: "DateTimeFormatMissingInputFieldError",
  2050: "DateTimeFormatSkeletonError",
  2051: "DateTimeFormatUnsupportedFieldError",
  2052: "DateTimeFormatUnsupportedOptionsError",
  2053: "DateTimeFormatMissingWeekdaySymbolError",
  2054: "DateTimeFormatMissingMonthSymbolError",
  2055: "DateTimeFormatFixedDecimalError",
  2056: "DateTimeFormatMismatchedAnyCalendarError",
  2057: "DateTimeFormatMismatchedCalendarLocaleError",
};

export const ICU4XError = {
  "UnknownError": "UnknownError",
  "WriteableError": "WriteableError",
  "OutOfBoundsError": "OutOfBoundsError",
  "DataMissingDataKeyError": "DataMissingDataKeyError",
  "DataMissingVariantError": "DataMissingVariantError",
  "DataMissingLocaleError": "DataMissingLocaleError",
  "DataNeedsVariantError": "DataNeedsVariantError",
  "DataNeedsLocaleError": "DataNeedsLocaleError",
  "DataExtraneousLocaleError": "DataExtraneousLocaleError",
  "DataFilteredResourceError": "DataFilteredResourceError",
  "DataMismatchedTypeError": "DataMismatchedTypeError",
  "DataMissingPayloadError": "DataMissingPayloadError",
  "DataInvalidStateError": "DataInvalidStateError",
  "DataCustomError": "DataCustomError",
  "DataIoError": "DataIoError",
  "DataUnavailableBufferFormatError": "DataUnavailableBufferFormatError",
  "LocaleUndefinedSubtagError": "LocaleUndefinedSubtagError",
  "LocaleParserError": "LocaleParserError",
  "DataStructValidityError": "DataStructValidityError",
  "PropertyUnknownScriptIdError": "PropertyUnknownScriptIdError",
  "PropertyUnknownGeneralCategoryGroupError": "PropertyUnknownGeneralCategoryGroupError",
  "DecimalLimitError": "DecimalLimitError",
  "DecimalSyntaxError": "DecimalSyntaxError",
  "PluralParserError": "PluralParserError",
  "DateTimeParseError": "DateTimeParseError",
  "DateTimeOverflowError": "DateTimeOverflowError",
  "DateTimeUnderflowError": "DateTimeUnderflowError",
  "DateTimeInvalidTimeZoneOffsetError": "DateTimeInvalidTimeZoneOffsetError",
  "DateTimeOutOfRangeError": "DateTimeOutOfRangeError",
  "DateTimeMissingInputError": "DateTimeMissingInputError",
  "DateTimeFormatPatternError": "DateTimeFormatPatternError",
  "DateTimeFormatMissingInputFieldError": "DateTimeFormatMissingInputFieldError",
  "DateTimeFormatSkeletonError": "DateTimeFormatSkeletonError",
  "DateTimeFormatUnsupportedFieldError": "DateTimeFormatUnsupportedFieldError",
  "DateTimeFormatUnsupportedOptionsError": "DateTimeFormatUnsupportedOptionsError",
  "DateTimeFormatMissingWeekdaySymbolError": "DateTimeFormatMissingWeekdaySymbolError",
  "DateTimeFormatMissingMonthSymbolError": "DateTimeFormatMissingMonthSymbolError",
  "DateTimeFormatFixedDecimalError": "DateTimeFormatFixedDecimalError",
  "DateTimeFormatMismatchedAnyCalendarError": "DateTimeFormatMismatchedAnyCalendarError",
  "DateTimeFormatMismatchedCalendarLocaleError": "DateTimeFormatMismatchedCalendarLocaleError",
};
