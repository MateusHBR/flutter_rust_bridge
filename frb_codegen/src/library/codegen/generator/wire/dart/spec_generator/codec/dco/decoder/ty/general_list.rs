use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for GeneralListWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        format!(
            "return (raw as List<dynamic>).map(dco_decode_{}).toList();",
            self.ir.inner.safe_ident()
        )
    }
}
