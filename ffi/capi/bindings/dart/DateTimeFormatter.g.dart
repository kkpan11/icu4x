// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X DateFormatter object capable of formatting a [`DateTime`] as a string,
/// using some calendar specified at runtime in the locale.
///
/// See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html) for more information.
final class DateTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  DateTimeFormatter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XDateTimeFormatter_destroy));

  /// Creates a new [`DateTimeFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
  ///
  /// Throws [Error] on failure.
  factory DateTimeFormatter.withLengths(DataProvider provider, Locale locale, DateLength dateLength, TimeLength timeLength) {
    final result = _ICU4XDateTimeFormatter_create_with_lengths(provider._ffi, locale._ffi, dateLength.index, timeLength.index);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return DateTimeFormatter._fromFfi(result.union.ok, []);
  }

  /// Formats a [`DateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatDatetime(DateTime value) {
    final writeable = _Writeable();
    final result = _ICU4XDateTimeFormatter_format_datetime(_ffi, value._ffi, writeable._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return writeable.finalize();
  }

  /// Formats a [`IsoDateTime`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatIsoDatetime(IsoDateTime value) {
    final writeable = _Writeable();
    final result = _ICU4XDateTimeFormatter_format_iso_datetime(_ffi, value._ffi, writeable._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return writeable.finalize();
  }
}

@meta.ResourceIdentifier('ICU4XDateTimeFormatter_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XDateTimeFormatter_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XDateTimeFormatter_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XDateTimeFormatter_create_with_lengths')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32, ffi.Int32)>(isLeaf: true, symbol: 'ICU4XDateTimeFormatter_create_with_lengths')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XDateTimeFormatter_create_with_lengths(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int dateLength, int timeLength);

@meta.ResourceIdentifier('ICU4XDateTimeFormatter_format_datetime')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDateTimeFormatter_format_datetime')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDateTimeFormatter_format_datetime(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> writeable);

@meta.ResourceIdentifier('ICU4XDateTimeFormatter_format_iso_datetime')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDateTimeFormatter_format_iso_datetime')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDateTimeFormatter_format_iso_datetime(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> writeable);
