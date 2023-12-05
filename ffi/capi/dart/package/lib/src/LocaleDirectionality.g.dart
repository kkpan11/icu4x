// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `LocaleDirectionality`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html) for more information.
final class LocaleDirectionality implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  LocaleDirectionality._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XLocaleDirectionality_destroy'));

  /// Construct a new ICU4XLocaleDirectionality instance
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory LocaleDirectionality(DataProvider provider) {
    final result = _ICU4XLocaleDirectionality_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return LocaleDirectionality._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocaleDirectionality_create =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleDirectionality_create')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Construct a new ICU4XLocaleDirectionality instance with a custom expander
  ///
  /// See the [Rust documentation for `new_with_expander`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.new_with_expander) for more information.
  ///
  /// Throws [Error] on failure.
  factory LocaleDirectionality.withExpander(DataProvider provider, LocaleExpander expander) {
    final result = _ICU4XLocaleDirectionality_create_with_expander(provider._underlying, expander._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return LocaleDirectionality._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocaleDirectionality_create_with_expander =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleDirectionality_create_with_expander')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.get) for more information.
  LocaleDirection operator [](Locale locale) {
    final result = _ICU4XLocaleDirectionality_get(_underlying, locale._underlying);
    return LocaleDirection.values[result];
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocaleDirectionality_get =
    _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleDirectionality_get')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `is_left_to_right`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.is_left_to_right) for more information.
  bool isLeftToRight(Locale locale) {
    final result = _ICU4XLocaleDirectionality_is_left_to_right(_underlying, locale._underlying);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocaleDirectionality_is_left_to_right =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleDirectionality_is_left_to_right')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `is_right_to_left`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.is_right_to_left) for more information.
  bool isRightToLeft(Locale locale) {
    final result = _ICU4XLocaleDirectionality_is_right_to_left(_underlying, locale._underlying);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocaleDirectionality_is_right_to_left =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleDirectionality_is_right_to_left')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}