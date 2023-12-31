// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.8.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'exception_twin_sync.freezed.dart';

int funcReturnErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcReturnErrorTwinSync(hint: hint);

int funcTypeFalliblePanicTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcTypeFalliblePanicTwinSync(hint: hint);

int funcTypeInfalliblePanicTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcTypeInfalliblePanicTwinSync(hint: hint);

int customEnumErrorReturnOkTwinSync({required int arg, dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnOkTwinSync(arg: arg, hint: hint);

void customEnumErrorPanicTwinSync({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorPanicTwinSync(hint: hint);

int customEnumErrorReturnErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnErrorTwinSync(hint: hint);

void customNestedErrorReturnErrorTwinSync(
        {required CustomNestedErrorOuterTwinSync arg, dynamic hint}) =>
    RustLib.instance.api
        .customNestedErrorReturnErrorTwinSync(arg: arg, hint: hint);

void customStructErrorReturnErrorTwinSync(
        {required CustomStructErrorTwinSync arg, dynamic hint}) =>
    RustLib.instance.api
        .customStructErrorReturnErrorTwinSync(arg: arg, hint: hint);

int returnErrCustomErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.returnErrCustomErrorTwinSync(hint: hint);

int returnOkCustomErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.returnOkCustomErrorTwinSync(hint: hint);

int returnErrorVariantTwinSync({required int variant, dynamic hint}) =>
    RustLib.instance.api
        .returnErrorVariantTwinSync(variant: variant, hint: hint);

void returnCustomNestedError1TwinSync({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError1TwinSync(hint: hint);

void returnCustomNestedError1Variant1TwinSync({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError1Variant1TwinSync(hint: hint);

void returnCustomNestedError2TwinSync({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError2TwinSync(hint: hint);

void returnCustomStructErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.returnCustomStructErrorTwinSync(hint: hint);

int returnCustomStructOkTwinSync({dynamic hint}) =>
    RustLib.instance.api.returnCustomStructOkTwinSync(hint: hint);

void throwAnyhowTwinSync({dynamic hint}) =>
    RustLib.instance.api.throwAnyhowTwinSync(hint: hint);

void panicWithCustomResultTwinSync({dynamic hint}) =>
    RustLib.instance.api.panicWithCustomResultTwinSync(hint: hint);

Stream<String> streamSinkThrowAnyhowTwinSync({dynamic hint}) =>
    RustLib.instance.api.streamSinkThrowAnyhowTwinSync(hint: hint);

@freezed
sealed class CustomEnumErrorTwinSync
    with _$CustomEnumErrorTwinSync
    implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinSync.one({
    required String message,
    required String backtrace,
  }) = CustomEnumErrorTwinSync_One;
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinSync.two({
    required int message,
    required String backtrace,
  }) = CustomEnumErrorTwinSync_Two;
}

@freezed
sealed class CustomErrorTwinSync
    with _$CustomErrorTwinSync
    implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomErrorTwinSync.error0({
    required String e,
    required String backtrace,
  }) = CustomErrorTwinSync_Error0;
  @Implements<FrbBacktracedException>()
  const factory CustomErrorTwinSync.error1({
    required int e,
    required String backtrace,
  }) = CustomErrorTwinSync_Error1;
}

@freezed
sealed class CustomNestedError1TwinSync
    with _$CustomNestedError1TwinSync
    implements FrbException {
  const factory CustomNestedError1TwinSync.customNested1(
    String field0,
  ) = CustomNestedError1TwinSync_CustomNested1;
  const factory CustomNestedError1TwinSync.errorNested(
    CustomNestedError2TwinSync field0,
  ) = CustomNestedError1TwinSync_ErrorNested;
}

@freezed
sealed class CustomNestedError2TwinSync with _$CustomNestedError2TwinSync {
  const factory CustomNestedError2TwinSync.customNested2(
    String field0,
  ) = CustomNestedError2TwinSync_CustomNested2;
  const factory CustomNestedError2TwinSync.customNested2Number(
    int field0,
  ) = CustomNestedError2TwinSync_CustomNested2Number;
}

@freezed
sealed class CustomNestedErrorInnerTwinSync
    with _$CustomNestedErrorInnerTwinSync {
  const factory CustomNestedErrorInnerTwinSync.three(
    String field0,
  ) = CustomNestedErrorInnerTwinSync_Three;
  const factory CustomNestedErrorInnerTwinSync.four(
    int field0,
  ) = CustomNestedErrorInnerTwinSync_Four;
}

@freezed
sealed class CustomNestedErrorOuterTwinSync
    with _$CustomNestedErrorOuterTwinSync {
  const factory CustomNestedErrorOuterTwinSync.one(
    String field0,
  ) = CustomNestedErrorOuterTwinSync_One;
  const factory CustomNestedErrorOuterTwinSync.two(
    CustomNestedErrorInnerTwinSync field0,
  ) = CustomNestedErrorOuterTwinSync_Two;
}

class CustomStructErrorAnotherTwinSync implements FrbException {
  final String message;

  const CustomStructErrorAnotherTwinSync({
    required this.message,
  });

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorAnotherTwinSync &&
          runtimeType == other.runtimeType &&
          message == other.message;
}

class CustomStructErrorTwinSync {
  final String a;

  const CustomStructErrorTwinSync({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorTwinSync &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class CustomStructTwinSync {
  final String message;

  const CustomStructTwinSync({
    required this.message,
  });

  static CustomStructTwinSync newTwinSync(
          {required String message, dynamic hint}) =>
      RustLib.instance.api
          .customStructTwinSyncNewTwinSync(message: message, hint: hint);

  void nonstaticReturnCustomStructErrorTwinSync({dynamic hint}) =>
      RustLib.instance.api
          .customStructTwinSyncNonstaticReturnCustomStructErrorTwinSync(
        that: this,
      );

  int nonstaticReturnCustomStructOkTwinSync({dynamic hint}) =>
      RustLib.instance.api
          .customStructTwinSyncNonstaticReturnCustomStructOkTwinSync(
        that: this,
      );

  static void staticReturnCustomStructErrorTwinSync({dynamic hint}) => RustLib
      .instance.api
      .customStructTwinSyncStaticReturnCustomStructErrorTwinSync(hint: hint);

  static int staticReturnCustomStructOkTwinSync({dynamic hint}) =>
      RustLib.instance.api
          .customStructTwinSyncStaticReturnCustomStructOkTwinSync(hint: hint);

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructTwinSync &&
          runtimeType == other.runtimeType &&
          message == other.message;
}

class SomeStructTwinSync {
  final int value;

  const SomeStructTwinSync({
    required this.value,
  });

  static SomeStructTwinSync newTwinSync({required int value, dynamic hint}) =>
      RustLib.instance.api
          .someStructTwinSyncNewTwinSync(value: value, hint: hint);

  int nonStaticReturnErrCustomErrorTwinSync({dynamic hint}) =>
      RustLib.instance.api
          .someStructTwinSyncNonStaticReturnErrCustomErrorTwinSync(
        that: this,
      );

  int nonStaticReturnOkCustomErrorTwinSync({dynamic hint}) =>
      RustLib.instance.api
          .someStructTwinSyncNonStaticReturnOkCustomErrorTwinSync(
        that: this,
      );

  static int staticReturnErrCustomErrorTwinSync({dynamic hint}) =>
      RustLib.instance.api
          .someStructTwinSyncStaticReturnErrCustomErrorTwinSync(hint: hint);

  static int staticReturnOkCustomErrorTwinSync({dynamic hint}) =>
      RustLib.instance.api
          .someStructTwinSyncStaticReturnOkCustomErrorTwinSync(hint: hint);

  @override
  int get hashCode => value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SomeStructTwinSync &&
          runtimeType == other.runtimeType &&
          value == other.value;
}
