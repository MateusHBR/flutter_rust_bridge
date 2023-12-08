// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 1.82.4.

// ignore_for_file: unused_import, unused_element, duplicate_ignore

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
  Future<int> hello({required int a, required int b, dynamic hint});

  Future<int> minimalAdder({required int a, required int b, dynamic hint});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  Future<int> hello({required int a, required int b, dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer();
        serializer.serialize_TODO(port_);
        serializer.serialize_TODO(a);
        serializer.serialize_TODO(b);
        final (ptr_, len_) = serializer.createLeakedNative();
        // TODO free pointer
        return wire.wire_hello(ptr_, len_);
      },
      codec: SseCodec(
        parseSuccessData: _sse_decode_i_32,
        parseErrorData: null,
      ),
      constMeta: kHelloConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHelloConstMeta => const TaskConstMeta(
        debugName: "hello",
        argNames: ["a", "b"],
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
        parseSuccessData: _dco_decode_i_32,
        parseErrorData: null,
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

  int _dco_decode_i_32(dynamic raw) {
    return raw as int;
  }
}

// Section: rust2dart

int _sse_decode_i_32(SseDeserializer deserializer) {
  return TODO_depend_on_serializer;
}

// Section: dart2rust

int cst_encode_i_32(int raw) {
  return raw;
}

void _sse_encode_i_32(int self, SseSerializer serializer) {
  return TODO_depend_on_serializer;
}
