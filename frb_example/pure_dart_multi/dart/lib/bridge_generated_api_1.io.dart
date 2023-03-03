// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.67.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated_api_1.dart';
export 'bridge_generated_api_1.dart';
import 'dart:ffi' as ffi;

class ApiClass1Platform extends FlutterRustBridgeBase<ApiClass1Wire> {
  ApiClass1Platform(ffi.DynamicLibrary dylib) : super(ApiClass1Wire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_CrossSharedStruct> api2wire_box_autoadd_cross_shared_struct(CrossSharedStruct raw) {
    final ptr = inner.new_box_autoadd_cross_shared_struct_0();
    _api_fill_to_wire_cross_shared_struct(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_OnlyForApi1Struct> api2wire_box_autoadd_only_for_api_1_struct(OnlyForApi1Struct raw) {
    final ptr = inner.new_box_autoadd_only_for_api_1_struct_0();
    _api_fill_to_wire_only_for_api_1_struct(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_SharedStruct> api2wire_box_autoadd_shared_struct(SharedStruct raw) {
    final ptr = inner.new_box_autoadd_shared_struct_0();
    _api_fill_to_wire_shared_struct(raw, ptr.ref);
    return ptr;
  }

  @protected
  int api2wire_u64(int raw) {
    return raw;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

// Section: api_fill_to_wire

  void _api_fill_to_wire_box_autoadd_cross_shared_struct(
      CrossSharedStruct apiObj, ffi.Pointer<wire_CrossSharedStruct> wireObj) {
    _api_fill_to_wire_cross_shared_struct(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_only_for_api_1_struct(
      OnlyForApi1Struct apiObj, ffi.Pointer<wire_OnlyForApi1Struct> wireObj) {
    _api_fill_to_wire_only_for_api_1_struct(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_shared_struct(SharedStruct apiObj, ffi.Pointer<wire_SharedStruct> wireObj) {
    _api_fill_to_wire_shared_struct(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_cross_shared_struct(CrossSharedStruct apiObj, wire_CrossSharedStruct wireObj) {
    wireObj.name = api2wire_String(apiObj.name);
  }

  void _api_fill_to_wire_only_for_api_1_struct(OnlyForApi1Struct apiObj, wire_OnlyForApi1Struct wireObj) {
    wireObj.id = api2wire_i16(apiObj.id);
    wireObj.num = api2wire_f64(apiObj.num);
    wireObj.name = api2wire_String(apiObj.name);
  }

  void _api_fill_to_wire_shared_struct(SharedStruct apiObj, wire_SharedStruct wireObj) {
    wireObj.id = api2wire_i32(apiObj.id);
    wireObj.num = api2wire_f64(apiObj.num);
    wireObj.name = api2wire_String(apiObj.name);
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class ApiClass1Wire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  ApiClass1Wire(ffi.DynamicLibrary dynamicLibrary) : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  ApiClass1Wire.fromLookup(ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) lookup)
      : _lookup = lookup;

  ffi.Pointer<wire_OnlyForApi2Struct> new_box_autoadd_only_for_api_2_struct_1() {
    return _new_box_autoadd_only_for_api_2_struct_1();
  }

  late final _new_box_autoadd_only_for_api_2_struct_1Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_OnlyForApi2Struct> Function()>>(
          'new_box_autoadd_only_for_api_2_struct_1');
  late final _new_box_autoadd_only_for_api_2_struct_1 =
      _new_box_autoadd_only_for_api_2_struct_1Ptr.asFunction<ffi.Pointer<wire_OnlyForApi2Struct> Function()>();

  ffi.Pointer<wire_SharedStruct> new_box_autoadd_shared_struct_1() {
    return _new_box_autoadd_shared_struct_1();
  }

  late final _new_box_autoadd_shared_struct_1Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_SharedStruct> Function()>>('new_box_autoadd_shared_struct_1');
  late final _new_box_autoadd_shared_struct_1 =
      _new_box_autoadd_shared_struct_1Ptr.asFunction<ffi.Pointer<wire_SharedStruct> Function()>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>('store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr.asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>('get_dart_object');
  late final _get_dart_object = _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>('drop_dart_object');
  late final _drop_dart_object = _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr = _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>('new_dart_opaque');
  late final _new_dart_opaque = _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>('init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr.asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_test_inbuilt_type_1(
    int port_,
    int a,
    double b,
  ) {
    return _wire_test_inbuilt_type_1(
      port_,
      a,
      b,
    );
  }

  late final _wire_test_inbuilt_type_1Ptr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32, ffi.Float)>>('wire_test_inbuilt_type_1');
  late final _wire_test_inbuilt_type_1 = _wire_test_inbuilt_type_1Ptr.asFunction<void Function(int, int, double)>();

  void wire_test_string_1(
    int port_,
    ffi.Pointer<wire_uint_8_list> s,
    int i,
  ) {
    return _wire_test_string_1(
      port_,
      s,
      i,
    );
  }

  late final _wire_test_string_1Ptr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>, ffi.Uint64)>>(
          'wire_test_string_1');
  late final _wire_test_string_1 =
      _wire_test_string_1Ptr.asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>, int)>();

  void wire_test_shared_struct_1(
    int port_,
    ffi.Pointer<wire_SharedStruct> custom,
    ffi.Pointer<wire_uint_8_list> s,
    int i,
  ) {
    return _wire_test_shared_struct_1(
      port_,
      custom,
      s,
      i,
    );
  }

  late final _wire_test_shared_struct_1Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_SharedStruct>, ffi.Pointer<wire_uint_8_list>,
              ffi.Int32)>>('wire_test_shared_struct_1');
  late final _wire_test_shared_struct_1 = _wire_test_shared_struct_1Ptr
      .asFunction<void Function(int, ffi.Pointer<wire_SharedStruct>, ffi.Pointer<wire_uint_8_list>, int)>();

  void wire_test_unique_struct_1(
    int port_,
    ffi.Pointer<wire_OnlyForApi1Struct> custom,
    ffi.Pointer<wire_uint_8_list> s,
    int i,
  ) {
    return _wire_test_unique_struct_1(
      port_,
      custom,
      s,
      i,
    );
  }

  late final _wire_test_unique_struct_1Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_OnlyForApi1Struct>, ffi.Pointer<wire_uint_8_list>,
              ffi.Int16)>>('wire_test_unique_struct_1');
  late final _wire_test_unique_struct_1 = _wire_test_unique_struct_1Ptr
      .asFunction<void Function(int, ffi.Pointer<wire_OnlyForApi1Struct>, ffi.Pointer<wire_uint_8_list>, int)>();

  void wire_test_cross_shared_struct_1(
    int port_,
    ffi.Pointer<wire_CrossSharedStruct> custom,
  ) {
    return _wire_test_cross_shared_struct_1(
      port_,
      custom,
    );
  }

  late final _wire_test_cross_shared_struct_1Ptr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_CrossSharedStruct>)>>(
          'wire_test_cross_shared_struct_1');
  late final _wire_test_cross_shared_struct_1 =
      _wire_test_cross_shared_struct_1Ptr.asFunction<void Function(int, ffi.Pointer<wire_CrossSharedStruct>)>();

  ffi.Pointer<wire_CrossSharedStruct> new_box_autoadd_cross_shared_struct_0() {
    return _new_box_autoadd_cross_shared_struct_0();
  }

  late final _new_box_autoadd_cross_shared_struct_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_CrossSharedStruct> Function()>>(
          'new_box_autoadd_cross_shared_struct_0');
  late final _new_box_autoadd_cross_shared_struct_0 =
      _new_box_autoadd_cross_shared_struct_0Ptr.asFunction<ffi.Pointer<wire_CrossSharedStruct> Function()>();

  ffi.Pointer<wire_OnlyForApi1Struct> new_box_autoadd_only_for_api_1_struct_0() {
    return _new_box_autoadd_only_for_api_1_struct_0();
  }

  late final _new_box_autoadd_only_for_api_1_struct_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_OnlyForApi1Struct> Function()>>(
          'new_box_autoadd_only_for_api_1_struct_0');
  late final _new_box_autoadd_only_for_api_1_struct_0 =
      _new_box_autoadd_only_for_api_1_struct_0Ptr.asFunction<ffi.Pointer<wire_OnlyForApi1Struct> Function()>();

  ffi.Pointer<wire_SharedStruct> new_box_autoadd_shared_struct_0() {
    return _new_box_autoadd_shared_struct_0();
  }

  late final _new_box_autoadd_shared_struct_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_SharedStruct> Function()>>('new_box_autoadd_shared_struct_0');
  late final _new_box_autoadd_shared_struct_0 =
      _new_box_autoadd_shared_struct_0Ptr.asFunction<ffi.Pointer<wire_SharedStruct> Function()>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_uint_8_list> Function(ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr.asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>('free_WireSyncReturn');
  late final _free_WireSyncReturn = _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

class _Dart_Handle extends ffi.Opaque {}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

class wire_OnlyForApi2Struct extends ffi.Struct {
  @ffi.Int64()
  external int id;

  @ffi.Double()
  external double num;

  external ffi.Pointer<wire_uint_8_list> name;
}

class wire_SharedStruct extends ffi.Struct {
  @ffi.Int32()
  external int id;

  @ffi.Double()
  external double num;

  external ffi.Pointer<wire_uint_8_list> name;
}

class wire_OnlyForApi1Struct extends ffi.Struct {
  @ffi.Int16()
  external int id;

  @ffi.Double()
  external double num;

  external ffi.Pointer<wire_uint_8_list> name;
}

class wire_CrossSharedStruct extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> name;
}

typedef DartPostCObjectFnType = ffi.Pointer<ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
