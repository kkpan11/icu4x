// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An object capable of mapping from an IANA time zone ID to a BCP-47 ID.
///
/// This can be used via `try_set_iana_time_zone_id()` on [`CustomTimeZone`].
///
/// [`CustomTimeZone`]: crate::timezone::ffi::ICU4XCustomTimeZone
///
/// See the [Rust documentation for `IanaToBcp47Mapper`](https://docs.rs/icu/latest/icu/timezone/struct.IanaToBcp47Mapper.html) for more information.
final class IanaToBcp47Mapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  IanaToBcp47Mapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XIanaToBcp47Mapper_destroy));

  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/timezone/struct.IanaToBcp47Mapper.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory IanaToBcp47Mapper(DataProvider provider) {
    final result = _ICU4XIanaToBcp47Mapper_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return IanaToBcp47Mapper._(result.union.ok);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XIanaToBcp47Mapper_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XIanaToBcp47Mapper_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XIanaToBcp47Mapper_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XIanaToBcp47Mapper_create(ffi.Pointer<ffi.Opaque> provider);