// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X DateFormatter object capable of formatting a [`Date`] as a string,
/// using some calendar specified at runtime in the locale.
///
/// See the [Rust documentation for `DateFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html) for more information.
final class DateFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  DateFormatter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XDateFormatter_destroy));

  /// Creates a new [`DateFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new_with_length`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.try_new_with_length) for more information.
  ///
  /// Throws [Error] on failure.
  factory DateFormatter.withLength(DataProvider provider, Locale locale, DateLength dateLength) {
    final result = _ICU4XDateFormatter_create_with_length(provider._ffi, locale._ffi, dateLength.index);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return DateFormatter._fromFfi(result.union.ok, []);
  }

  /// Formats a [`Date`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatDate(Date value) {
    final writeable = _Writeable();
    final result = _ICU4XDateFormatter_format_date(_ffi, value._ffi, writeable._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return writeable.finalize();
  }

  /// Formats a [`IsoDate`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatIsoDate(IsoDate value) {
    final writeable = _Writeable();
    final result = _ICU4XDateFormatter_format_iso_date(_ffi, value._ffi, writeable._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return writeable.finalize();
  }

  /// Formats a [`DateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatDatetime(DateTime value) {
    final writeable = _Writeable();
    final result = _ICU4XDateFormatter_format_datetime(_ffi, value._ffi, writeable._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return writeable.finalize();
  }

  /// Formats a [`IsoDateTime`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  ///
  /// Throws [Error] on failure.
  String formatIsoDatetime(IsoDateTime value) {
    final writeable = _Writeable();
    final result = _ICU4XDateFormatter_format_iso_datetime(_ffi, value._ffi, writeable._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return writeable.finalize();
  }
}

@meta.ResourceIdentifier('ICU4XDateFormatter_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XDateFormatter_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XDateFormatter_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XDateFormatter_create_with_length')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ICU4XDateFormatter_create_with_length')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XDateFormatter_create_with_length(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int dateLength);

@meta.ResourceIdentifier('ICU4XDateFormatter_format_date')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDateFormatter_format_date')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDateFormatter_format_date(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> writeable);

@meta.ResourceIdentifier('ICU4XDateFormatter_format_iso_date')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDateFormatter_format_iso_date')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDateFormatter_format_iso_date(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> writeable);

@meta.ResourceIdentifier('ICU4XDateFormatter_format_datetime')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDateFormatter_format_datetime')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDateFormatter_format_datetime(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> writeable);

@meta.ResourceIdentifier('ICU4XDateFormatter_format_iso_datetime')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDateFormatter_format_iso_datetime')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XDateFormatter_format_iso_datetime(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> writeable);
