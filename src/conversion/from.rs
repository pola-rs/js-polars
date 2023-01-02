use wasm_bindgen::prelude::*;


#[repr(u32)]
#[wasm_bindgen(js_name=DataType)]
pub enum JsDataType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Bool,
    Utf8,
    List,
    Date,
    Datetime,
    Time,
    Object,
    Categorical,
}
