// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _CollatorOptionsFfi extends ffi.Struct {
  @ffi.Int32()
  external int strength;
  @ffi.Int32()
  external int alternateHandling;
  @ffi.Int32()
  external int caseFirst;
  @ffi.Int32()
  external int maxVariable;
  @ffi.Int32()
  external int caseLevel;
  @ffi.Int32()
  external int numeric;
  @ffi.Int32()
  external int backwardSecondLevel;
}

/// See the [Rust documentation for `CollatorOptions`](https://docs.rs/icu/latest/icu/collator/struct.CollatorOptions.html) for more information.
final class CollatorOptions {
  CollatorStrength strength;
  CollatorAlternateHandling alternateHandling;
  CollatorCaseFirst caseFirst;
  CollatorMaxVariable maxVariable;
  CollatorCaseLevel caseLevel;
  CollatorNumeric numeric;
  CollatorBackwardSecondLevel backwardSecondLevel;

  CollatorOptions({required this.strength, required this.alternateHandling, required this.caseFirst, required this.maxVariable, required this.caseLevel, required this.numeric, required this.backwardSecondLevel});

  // ignore: unused_element
  // Internal constructor from FFI.
  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  CollatorOptions._(_CollatorOptionsFfi underlying) :
    strength = CollatorStrength.values[underlying.strength],
    alternateHandling = CollatorAlternateHandling.values[underlying.alternateHandling],
    caseFirst = CollatorCaseFirst.values[underlying.caseFirst],
    maxVariable = CollatorMaxVariable.values[underlying.maxVariable],
    caseLevel = CollatorCaseLevel.values[underlying.caseLevel],
    numeric = CollatorNumeric.values[underlying.numeric],
    backwardSecondLevel = CollatorBackwardSecondLevel.values[underlying.backwardSecondLevel];

  // ignore: unused_element
  _CollatorOptionsFfi _pointer(ffi.Allocator temp) {
    final pointer = temp<_CollatorOptionsFfi>();
    pointer.ref.strength = strength.index;
    pointer.ref.alternateHandling = alternateHandling.index;
    pointer.ref.caseFirst = caseFirst.index;
    pointer.ref.maxVariable = maxVariable.index;
    pointer.ref.caseLevel = caseLevel.index;
    pointer.ref.numeric = numeric.index;
    pointer.ref.backwardSecondLevel = backwardSecondLevel.index;
    return pointer.ref;
  }

  @override
  bool operator ==(Object other) =>
      other is CollatorOptions &&
      other.strength == this.strength &&
      other.alternateHandling == this.alternateHandling &&
      other.caseFirst == this.caseFirst &&
      other.maxVariable == this.maxVariable &&
      other.caseLevel == this.caseLevel &&
      other.numeric == this.numeric &&
      other.backwardSecondLevel == this.backwardSecondLevel;

  @override
  int get hashCode => Object.hashAll([
        this.strength,
        this.alternateHandling,
        this.caseFirst,
        this.maxVariable,
        this.caseLevel,
        this.numeric,
        this.backwardSecondLevel,
      ]);
}