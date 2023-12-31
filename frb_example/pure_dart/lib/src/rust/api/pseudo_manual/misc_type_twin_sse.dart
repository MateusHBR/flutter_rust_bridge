// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.8.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<String> funcStringTwinSse({required String arg, dynamic hint}) =>
    RustLib.instance.api.funcStringTwinSse(arg: arg, hint: hint);

Future<void> funcReturnUnitTwinSse({dynamic hint}) =>
    RustLib.instance.api.funcReturnUnitTwinSse(hint: hint);

Future<List<MySize>> handleListOfStructTwinSse(
        {required List<MySize> l, dynamic hint}) =>
    RustLib.instance.api.handleListOfStructTwinSse(l: l, hint: hint);

Future<List<String>> handleStringListTwinSse(
        {required List<String> names, dynamic hint}) =>
    RustLib.instance.api.handleStringListTwinSse(names: names, hint: hint);

Future<EmptyTwinSse> emptyStructTwinSse(
        {required EmptyTwinSse empty, dynamic hint}) =>
    RustLib.instance.api.emptyStructTwinSse(empty: empty, hint: hint);

class EmptyTwinSse {
  const EmptyTwinSse();

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is EmptyTwinSse && runtimeType == other.runtimeType;
}
