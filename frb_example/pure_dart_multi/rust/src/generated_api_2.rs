#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.67.0.

use crate::api_2::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::custom::CrossSharedStruct;
use crate::custom::OnlyForApi2Struct;
use crate::custom::SharedStruct;

// Section: wire functions

fn wire_test_inbuilt_type_2_impl(
    port_: MessagePort,
    a: impl Wire2Api<i32> + UnwindSafe,
    b: impl Wire2Api<f32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "test_inbuilt_type_2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_a = a.wire2api();
            let api_b = b.wire2api();
            move |task_callback| Ok(test_inbuilt_type_2(api_a, api_b))
        },
    )
}
fn wire_test_string_2_impl(
    port_: MessagePort,
    s: impl Wire2Api<String> + UnwindSafe,
    i: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "test_string_2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| Ok(test_string_2(api_s, api_i))
        },
    )
}
fn wire_test_shared_struct_2_impl(
    port_: MessagePort,
    custom: impl Wire2Api<SharedStruct> + UnwindSafe,
    s: impl Wire2Api<String> + UnwindSafe,
    i: impl Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "test_shared_struct_2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| Ok(test_shared_struct_2(api_custom, api_s, api_i))
        },
    )
}
fn wire_test_unique_struct_2_impl(
    port_: MessagePort,
    custom: impl Wire2Api<OnlyForApi2Struct> + UnwindSafe,
    s: impl Wire2Api<String> + UnwindSafe,
    i: impl Wire2Api<i64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "test_unique_struct_2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| Ok(test_unique_struct_2(api_custom, api_s, api_i))
        },
    )
}
fn wire_test_cross_shared_struct_2_impl(
    port_: MessagePort,
    name: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "test_cross_shared_struct_2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_name = name.wire2api();
            move |task_callback| Ok(test_cross_shared_struct_2(api_name))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}
impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for CrossSharedStruct {
    fn into_dart(self) -> support::DartAbi {
        vec![self.name.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for CrossSharedStruct {}

impl support::IntoDart for OnlyForApi2Struct {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_dart(),
            self.num.into_dart(),
            self.name.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OnlyForApi2Struct {}

impl support::IntoDart for SharedStruct {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_dart(),
            self.num.into_dart(),
            self.name.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SharedStruct {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "generated_api_2.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "generated_api_2.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
