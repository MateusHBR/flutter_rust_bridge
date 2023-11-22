use super::*;

// Section: impl_wire2api

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<Vec<bool>> for JsValue {
    fn wire2api(self) -> Vec<bool> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<f32>> for Box<[f32]> {
    fn wire2api(self) -> Vec<f32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i16>> for Box<[i16]> {
    fn wire2api(self) -> Vec<i16> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Vec<i32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i64>> for Box<[i64]> {
    fn wire2api(self) -> Vec<i64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i8>> for Box<[i8]> {
    fn wire2api(self) -> Vec<i8> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u16>> for Box<[u16]> {
    fn wire2api(self) -> Vec<u16> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u32>> for Box<[u32]> {
    fn wire2api(self) -> Vec<u32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u64>> for Box<[u64]> {
    fn wire2api(self) -> Vec<u64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for JsValue {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithCommentsTwinNormal {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for JsValue {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructWithCommentsTwinSync {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<f32> for JsValue {
    fn wire2api(self) -> f32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<f32>> for JsValue {
    fn wire2api(self) -> Vec<f32> {
        self.unchecked_into::<js_sys::Float32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<f64>> for JsValue {
    fn wire2api(self) -> Vec<f64> {
        self.unchecked_into::<js_sys::Float64Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<i16>> for JsValue {
    fn wire2api(self) -> Vec<i16> {
        self.unchecked_into::<js_sys::Int16Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<js_sys::Int32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<i64>> for JsValue {
    fn wire2api(self) -> Vec<i64> {
        let buf = self.dyn_into::<js_sys::BigInt64Array>().unwrap();
        let buf = js_sys::Uint8Array::new(&buf.buffer());
        support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<i8>> for JsValue {
    fn wire2api(self) -> Vec<i8> {
        self.unchecked_into::<js_sys::Int8Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u16>> for JsValue {
    fn wire2api(self) -> Vec<u16> {
        self.unchecked_into::<js_sys::Uint16Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u32>> for JsValue {
    fn wire2api(self) -> Vec<u32> {
        self.unchecked_into::<js_sys::Uint32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u64>> for JsValue {
    fn wire2api(self) -> Vec<u64> {
        let buf = self.dyn_into::<js_sys::BigUint64Array>().unwrap();
        let buf = js_sys::Uint8Array::new(&buf.buffer());
        support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<u16> for JsValue {
    fn wire2api(self) -> u16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: MessagePort,
    that: JsValue,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_: MessagePort) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_normal(port_: MessagePort) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_normal(port_: MessagePort) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_normal(port_: MessagePort) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: JsValue,
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_static_method_twin_sync() -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_sync() -> support::WireSyncReturn {
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_sync() -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_sync() -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: JsValue,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_normal(port_: MessagePort, arg: bool) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_normal(port_: MessagePort, arg: f32) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_normal(port_: MessagePort, arg: f64) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_normal(port_: MessagePort, arg: i16) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_normal(port_: MessagePort, arg: i32) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_normal(port_: MessagePort, arg: i64) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_normal(port_: MessagePort, arg: i8) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_normal(port_: MessagePort, arg: u16) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_normal(port_: MessagePort, arg: u32) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_normal(port_: MessagePort, arg: u64) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_normal(port_: MessagePort, arg: u8) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_normal(port_: MessagePort, arg: JsValue) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_normal(port_: MessagePort, arg: Box<[f32]>) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_normal(port_: MessagePort, arg: Box<[f64]>) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_normal(port_: MessagePort, arg: Box<[i16]>) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_normal(port_: MessagePort, arg: Box<[i32]>) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_normal(port_: MessagePort, arg: Box<[i64]>) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_normal(port_: MessagePort, arg: Box<[i8]>) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_normal(port_: MessagePort, arg: Box<[u16]>) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_normal(port_: MessagePort, arg: Box<[u32]>) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_normal(port_: MessagePort, arg: Box<[u64]>) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_normal(port_: MessagePort, arg: Box<[u8]>) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_sync(arg: JsValue) -> support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_sync(arg: Box<[f32]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_sync(arg: Box<[f64]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_sync(arg: Box<[i16]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_sync(arg: Box<[i32]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_sync(arg: Box<[i64]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_sync(arg: Box<[i8]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_sync(arg: Box<[u16]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_sync(arg: Box<[u32]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_sync(arg: Box<[u64]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_sync(arg: Box<[u8]>) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_sync(arg: bool) -> support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_sync(arg: f32) -> support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_sync(arg: f64) -> support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_sync(arg: i16) -> support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_sync(arg: i32) -> support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_sync(arg: i64) -> support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_sync(arg: i8) -> support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_sync(arg: u16) -> support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_sync(arg: u32) -> support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_sync(arg: u64) -> support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_sync(arg: u8) -> support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_normal(port_: MessagePort, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}
