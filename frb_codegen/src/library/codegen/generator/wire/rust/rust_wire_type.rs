use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorRustWireTypeTrait {
    fn rust_wire_type(&self, target: Target) -> String;
}

impl<'a> WireRustGeneratorRustWireTypeTrait for BoxedWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if target.is_wasm() && self.ir.inner.is_primitive() {
            "JsValue".into()
        } else {
            self.ir.inner.rust_wire_type(target)
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DartOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if target.is_wasm() {
            "JsValue"
        } else {
            "wire_DartOpaque"
        }
        .to_owned()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DelegateWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        match (&self.ir, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Io) => "wire_StringList".to_owned(),
            (IrTypeDelegate::StringList, Target::Wasm) => "JsValue".into(),
            _ => self.get_delegate().rust_wire_type(target),
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DynamicWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        panic!("Functions cannot receive dynamic parameters.")
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for EnumRefWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.ir.ident.0)
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for GeneralListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for OptionalWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if self.ir.inner.rust_wire_is_pointer(target)
            || target.is_wasm()
                && (self.ir.inner.is_js_value() || self.is_primitive() || self.is_boxed_primitive())
        {
            self.ir.inner.rust_wire_type(target)
        } else {
            format!("Option<{}>", self.ir.inner.rust_wire_type(target))
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for OptionalListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        match target {
            Target::Wasm => "JsValue".into(),
            Target::Io => format!("wire_{}", self.safe_ident()),
            Target::Common => unreachable!(),
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for PrimitiveWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        match self.ir {
            IrTypePrimitive::U8 => "u8",
            IrTypePrimitive::I8 => "i8",
            IrTypePrimitive::U16 => "u16",
            IrTypePrimitive::I16 => "i16",
            IrTypePrimitive::U32 => "u32",
            IrTypePrimitive::I32 => "i32",
            IrTypePrimitive::U64 => "u64",
            IrTypePrimitive::Unit => "unit",
            IrTypePrimitive::Usize => "usize",
            IrTypePrimitive::Isize => "isize",
            IrTypePrimitive::I64 => "i64",
            IrTypePrimitive::F32 => "f32",
            IrTypePrimitive::F64 => "f64",
            IrTypePrimitive::Bool => "bool",
        }
        .to_string()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for PrimitiveListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            match self.ir.primitive {
                IrTypePrimitive::Bool | IrTypePrimitive::Unit => "JsValue".into(),
                _ => format!("Box<[{}]>", self.ir.primitive.rust_api_type()),
            }
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for RecordWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if target.is_wasm() {
            "JsValue".to_string()
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for RustOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for StructRefWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.ir.ident.0)
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for UnencodableWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        unreachable!()
    }
}
