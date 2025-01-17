use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::body::generate_encode_or_decode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

pub(crate) struct SseWireRustCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for SseWireRustCodecEntrypoint
{
    fn generate(
        &self,
        context: WireRustGeneratorContext,
        types: &[IrType],
        mode: EncodeOrDecode,
    ) -> Option<WireRustCodecOutputSpec> {
        Some(generate_encode_or_decode(
            context.as_wire_rust_codec_sse_context(),
            types,
            mode,
        ))
    }
}

impl WireRustCodecEntrypointTrait<'_> for SseWireRustCodecEntrypoint {
    fn generate_func_params(
        &self,
        func: &IrFunc,
        _context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        Acc::new(|target| {
            let mut params = generate_platform_generalized_uint8list_params(target);

            if has_port_argument(func.mode) {
                params.insert(0, create_port_param(target));
            }

            params
        })
    }

    fn generate_func_call_decode(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> String {
        let primary = (func.inputs.iter())
            .map(|field| {
                let gen = WireRustGenerator::new(field.ty.clone(), context);

                let name = field.name.rust_style();
                let effective_rust_api_type = (gen.generate_wire_func_call_decode_type())
                    .unwrap_or_else(|| field.ty.rust_api_type());

                let mut expr =
                    format!("<{effective_rust_api_type}>::sse_decode(&mut deserializer)");
                if let Some(wrapper) = gen.generate_wire_func_call_decode_wrapper() {
                    expr = format!("{wrapper}({expr})");
                }

                format!("let api_{name} = {expr};")
            })
            .join("\n");
        format!(
            "
            let message = unsafe {{ flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(ptr_, rust_vec_len_, data_len_) }};
            let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            {primary}deserializer.end();"
        )
    }
}

pub(crate) fn create_port_param(target: TargetOrCommon) -> ExternFuncParam {
    let rust_type = match target {
        // NOTE Though in `io`, i64 == our MessagePort, but it will affect the cbindgen
        // and ffigen and make code tricker, so we manually write down "i64" here.
        TargetOrCommon::Io => "i64",
        TargetOrCommon::Common | TargetOrCommon::Web => {
            "flutter_rust_bridge::for_generated::MessagePort"
        }
    }
    .to_owned();
    ExternFuncParam {
        name: "port_".to_owned(),
        rust_type,
        dart_type: "NativePortType".to_owned(),
    }
}

pub(crate) fn generate_platform_generalized_uint8list_params(
    target: TargetOrCommon,
) -> Vec<ExternFuncParam> {
    vec![
        ExternFuncParam {
            name: "ptr_".to_owned(),
            rust_type: match target {
                TargetOrCommon::Common | TargetOrCommon::Web => {
                    "flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr".to_owned()
                }
                TargetOrCommon::Io => "*mut u8".to_owned(),
            },
            dart_type: "PlatformGeneralizedUint8ListPtr".to_owned(),
        },
        ExternFuncParam {
            name: "rust_vec_len_".to_owned(),
            rust_type: "i32".to_owned(),
            dart_type: "int".to_owned(),
        },
        ExternFuncParam {
            name: "data_len_".to_owned(),
            rust_type: "i32".to_owned(),
            dart_type: "int".to_owned(),
        },
    ]
}
