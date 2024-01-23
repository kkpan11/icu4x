// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An iterator over code point ranges, produced by `CodePointSetData` or
/// one of the `CodePointMapData` types
final class CodePointRangeIterator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  CodePointRangeIterator._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_CodePointRangeIterator_destroy));

  /// Advance the iterator by one and return the next range.
  ///
  /// If the iterator is out of items, `done` will be true
  CodePointRangeIteratorResult next() {
    final result = _CodePointRangeIterator_next(_underlying);
    return CodePointRangeIteratorResult._(result);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'CodePointRangeIterator_destroy')
// ignore: non_constant_identifier_names
external void _CodePointRangeIterator_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_CodePointRangeIteratorResultFfi Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'CodePointRangeIterator_next')
// ignore: non_constant_identifier_names
external _CodePointRangeIteratorResultFfi _CodePointRangeIterator_next(ffi.Pointer<ffi.Opaque> self);