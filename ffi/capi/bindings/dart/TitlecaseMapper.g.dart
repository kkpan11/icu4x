// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `TitlecaseMapper`](https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html) for more information.
final class TitlecaseMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  TitlecaseMapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XTitlecaseMapper_destroy));

  /// Construct a new `TitlecaseMapper` instance
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory TitlecaseMapper(DataProvider provider) {
    final result = _ICU4XTitlecaseMapper_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return TitlecaseMapper._(result.union.ok);
  }

  /// Returns the full titlecase mapping of the given string
  ///
  /// The `v1` refers to the version of the options struct, which may change as we add more options
  ///
  /// See the [Rust documentation for `titlecase_segment`](https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.titlecase_segment) for more information.
  ///
  /// Throws [Error] on failure.
  String titlecaseSegmentV1(String s, Locale locale, TitlecaseOptionsV1 options) {
    final temp = ffi2.Arena();
    final sView = s.utf8View;
    final writeable = _Writeable();
    final result = _ICU4XTitlecaseMapper_titlecase_segment_v1(_underlying, sView.pointer(temp), sView.length, locale._underlying, options._underlying, writeable._underlying);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XTitlecaseMapper_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XTitlecaseMapper_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XTitlecaseMapper_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XTitlecaseMapper_create(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Opaque>, _TitlecaseOptionsV1Ffi, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XTitlecaseMapper_titlecase_segment_v1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XTitlecaseMapper_titlecase_segment_v1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> sData, int sLength, ffi.Pointer<ffi.Opaque> locale, _TitlecaseOptionsV1Ffi options, ffi.Pointer<ffi.Opaque> writeable);