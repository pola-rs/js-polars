use super::JsExpr;
use crate::{extern_iterator, extern_struct};
use polars::prelude::Expr;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Expr")]
    pub type ExternExpr;

    #[wasm_bindgen(method, getter = ptr)]
    fn ptr(this: &ExternExpr) -> f64;

    #[wasm_bindgen(static_method_of = ExternExpr)]
    fn wrap(ptr: u32) -> ExternExpr;

    #[wasm_bindgen(typescript_type = "Expr[]")]
    pub type ExprArray;

}

extern_struct!(ExternExpr, JsExpr);
extern_iterator!(ExprArray, ExternExpr, JsExpr);

impl From<Expr> for JsExpr {
    fn from(s: Expr) -> JsExpr {
        JsExpr { inner: s }
    }
}
