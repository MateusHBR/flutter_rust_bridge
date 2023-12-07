mod boxed;
mod dart_fn;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod ownership;
mod primitive;
mod primitive_list;
mod record;
mod rust_auto_opaque;
mod rust_opaque;
mod structure;
mod unencodable;

use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorImplTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartCodecDcoGeneratorDecoderTrait {
    fn generate_impl_decode_body(&self) -> String;
}
