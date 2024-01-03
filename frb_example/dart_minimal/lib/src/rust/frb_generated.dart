// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.10.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables

import 'api/minimal.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
  }) async {
    await instance.initImpl(
      api: api,
      handler: handler,
      externalLibrary: externalLibrary,
    );
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  Future<void> executeRustInitializers() async {
    await api.initApp();
  }

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      kDefaultExternalLibraryLoaderConfig;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
    stem: 'frb_example_dart_minimal',
    ioDirectory: 'rust/target/release/',
    webPrefix: 'pkg/',
  );
}

abstract class RustLibApi extends BaseApi {
  Future<AnotherOpaqueType> bar({dynamic hint});

  Future<MyOpaqueTypeAnotherOpaqueType> foo({dynamic hint});

  Future<void> initApp({dynamic hint});

  Future<int> minimalAdder({required int a, required int b, dynamic hint});

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_AnotherOpaqueType;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_AnotherOpaqueType;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_AnotherOpaqueTypePtr;

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_MyOpaqueTypeAnotherOpaqueType;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_MyOpaqueTypeAnotherOpaqueType;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_MyOpaqueTypeAnotherOpaqueTypePtr;
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  Future<AnotherOpaqueType> bar({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire.wire_bar(port_);
      },
      codec: DcoCodec(
        decodeSuccessData:
            dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockAnotherOpaqueType,
        decodeErrorData: null,
      ),
      constMeta: kBarConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kBarConstMeta => const TaskConstMeta(
        debugName: "bar",
        argNames: [],
      );

  @override
  Future<MyOpaqueTypeAnotherOpaqueType> foo({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire.wire_foo(port_);
      },
      codec: DcoCodec(
        decodeSuccessData:
            dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType,
        decodeErrorData: null,
      ),
      constMeta: kFooConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFooConstMeta => const TaskConstMeta(
        debugName: "foo",
        argNames: [],
      );

  @override
  Future<void> initApp({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        return wire.wire_init_app(port_);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kInitAppConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kInitAppConstMeta => const TaskConstMeta(
        debugName: "init_app",
        argNames: [],
      );

  @override
  Future<int> minimalAdder({required int a, required int b, dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        var arg0 = cst_encode_i_32(a);
        var arg1 = cst_encode_i_32(b);
        return wire.wire_minimal_adder(port_, arg0, arg1);
      },
      codec: DcoCodec(
        decodeSuccessData: dco_decode_i_32,
        decodeErrorData: null,
      ),
      constMeta: kMinimalAdderConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMinimalAdderConstMeta => const TaskConstMeta(
        debugName: "minimal_adder",
        argNames: ["a", "b"],
      );

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_AnotherOpaqueType => wire
          .rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockAnotherOpaqueType;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_AnotherOpaqueType => wire
          .rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockAnotherOpaqueType;

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_MyOpaqueTypeAnotherOpaqueType => wire
          .rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_MyOpaqueTypeAnotherOpaqueType => wire
          .rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType;

  @protected
  AnotherOpaqueType
      dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockAnotherOpaqueType(
          dynamic raw) {
    return AnotherOpaqueType.dcoDecode(raw as List<dynamic>);
  }

  @protected
  MyOpaqueTypeAnotherOpaqueType
      dco_decode_Auto_Owned_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
          dynamic raw) {
    return MyOpaqueTypeAnotherOpaqueType.dcoDecode(raw as List<dynamic>);
  }

  @protected
  AnotherOpaqueType dco_decode_RustOpaque_stdsyncRwLockAnotherOpaqueType(
      dynamic raw) {
    return AnotherOpaqueType.dcoDecode(raw as List<dynamic>);
  }

  @protected
  MyOpaqueTypeAnotherOpaqueType
      dco_decode_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
          dynamic raw) {
    return MyOpaqueTypeAnotherOpaqueType.dcoDecode(raw as List<dynamic>);
  }

  @protected
  int dco_decode_i_32(dynamic raw) {
    return raw as int;
  }

  @protected
  void dco_decode_unit(dynamic raw) {
    return;
  }

  @protected
  int dco_decode_usize(dynamic raw) {
    return dcoDecodeI64OrU64(raw);
  }

  @protected
  AnotherOpaqueType
      sse_decode_Auto_Owned_RustOpaque_stdsyncRwLockAnotherOpaqueType(
          SseDeserializer deserializer) {
    return AnotherOpaqueType.sseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  MyOpaqueTypeAnotherOpaqueType
      sse_decode_Auto_Owned_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
          SseDeserializer deserializer) {
    return MyOpaqueTypeAnotherOpaqueType.sseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  AnotherOpaqueType sse_decode_RustOpaque_stdsyncRwLockAnotherOpaqueType(
      SseDeserializer deserializer) {
    return AnotherOpaqueType.sseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  MyOpaqueTypeAnotherOpaqueType
      sse_decode_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
          SseDeserializer deserializer) {
    return MyOpaqueTypeAnotherOpaqueType.sseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    return deserializer.buffer.getInt32();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {}

  @protected
  int sse_decode_usize(SseDeserializer deserializer) {
    return deserializer.buffer.getUint64();
  }

  @protected
  bool sse_decode_bool(SseDeserializer deserializer) {
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  PlatformPointer
      cst_encode_Auto_Owned_RustOpaque_stdsyncRwLockAnotherOpaqueType(
          AnotherOpaqueType raw) {
    // ignore: invalid_use_of_internal_member
    return raw.cstEncode(move: true);
  }

  @protected
  PlatformPointer
      cst_encode_Auto_Owned_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
          MyOpaqueTypeAnotherOpaqueType raw) {
    // ignore: invalid_use_of_internal_member
    return raw.cstEncode(move: true);
  }

  @protected
  PlatformPointer cst_encode_RustOpaque_stdsyncRwLockAnotherOpaqueType(
      AnotherOpaqueType raw) {
    // ignore: invalid_use_of_internal_member
    return raw.cstEncode();
  }

  @protected
  PlatformPointer
      cst_encode_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
          MyOpaqueTypeAnotherOpaqueType raw) {
    // ignore: invalid_use_of_internal_member
    return raw.cstEncode();
  }

  @protected
  int cst_encode_i_32(int raw) {
    return raw;
  }

  @protected
  void cst_encode_unit(void raw) {
    return raw;
  }

  @protected
  int cst_encode_usize(int raw) {
    return raw;
  }

  @protected
  void sse_encode_Auto_Owned_RustOpaque_stdsyncRwLockAnotherOpaqueType(
      AnotherOpaqueType self, SseSerializer serializer) {
    sse_encode_usize(self.sseEncode(move: true), serializer);
  }

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
          MyOpaqueTypeAnotherOpaqueType self, SseSerializer serializer) {
    sse_encode_usize(self.sseEncode(move: true), serializer);
  }

  @protected
  void sse_encode_RustOpaque_stdsyncRwLockAnotherOpaqueType(
      AnotherOpaqueType self, SseSerializer serializer) {
    sse_encode_usize(self.sseEncode(move: null), serializer);
  }

  @protected
  void sse_encode_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(
      MyOpaqueTypeAnotherOpaqueType self, SseSerializer serializer) {
    sse_encode_usize(self.sseEncode(move: null), serializer);
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    serializer.buffer.putInt32(self);
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {}

  @protected
  void sse_encode_usize(int self, SseSerializer serializer) {
    serializer.buffer.putUint64(self);
  }

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    serializer.buffer.putUint8(self ? 1 : 0);
  }
}
