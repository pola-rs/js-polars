use polars::lazy::dsl::Expr;
use polars::prelude::*;
use wasm_bindgen::prelude::*;

use super::JsExpr;

#[wasm_bindgen(js_name=StringNameSpace)]
pub struct JsStringNameSpace {
    pub(crate) inner: Expr,
}

#[wasm_bindgen(js_class=StringNameSpace)]
impl JsStringNameSpace {
    pub fn contains_literal(&self, literal: &str) -> JsExpr {
        self.inner.clone().str().contains_literal(literal).into()
    }

    pub fn contains(&self, pat: &str) -> JsExpr {
        self.inner.clone().str().contains(pat).into()
    }

    pub fn ends_with(&self, sub: &str) -> JsExpr {
        self.inner.clone().str().ends_with(sub).into()
    }

    pub fn starts_with(&self, sub: &str) -> JsExpr {
        self.inner.clone().str().starts_with(sub).into()
    }
    pub fn extract(&self, pat: &str, group_index: usize) -> JsExpr {
        self.inner.clone().str().extract(pat, group_index).into()
    }

    pub fn zfill(&self, alignment: usize) -> JsExpr {
        self.inner.clone().str().zfill(alignment).into()
    }
    pub fn ljust(&self, width: usize, fillchar: char) -> JsExpr {
        self.inner.clone().str().ljust(width, fillchar).into()
    }
    pub fn rjust(&self, width: usize, fillchar: char) -> JsExpr {
        self.inner.clone().str().rjust(width, fillchar).into()
    }

    pub fn extract_all(&self, pat: &JsExpr) -> JsExpr {
        self.inner
            .clone()
            .str()
            .extract_all(pat.inner.clone())
            .into()
    }

    pub fn count_match(&self, pat: &str) -> JsExpr {
        self.inner.clone().str().count_match(pat).into()
    }

    pub fn lengths(&self) -> JsExpr {
        let function = |s: Series| {
            let ca = s.utf8()?;
            Ok(ca.str_lengths().into_series())
        };
        self.inner
            .clone()
            .map(function, GetOutput::from_type(DataType::UInt32))
            .with_fmt("str.lengths")
            .into()
    }
    pub fn n_chars(&self) -> JsExpr {
        let function = |s: Series| {
            let ca = s.utf8()?;
            Ok(ca.str_n_chars().into_series())
        };
        self.inner
            .clone()
            .map(function, GetOutput::from_type(DataType::UInt32))
            .with_fmt("str.n_chars")
            .into()
    }

    fn hex_encode(&self) -> JsExpr {
        self.inner
            .clone()
            .map(
                move |s| s.utf8().map(|s| s.hex_encode().into_series()),
                GetOutput::same_type(),
            )
            .with_fmt("str.hex_encode")
            .into()
    }

    fn base64_encode(&self) -> JsExpr {
        self.inner
            .clone()
            .map(
                move |s| s.utf8().map(|s| s.base64_encode().into_series()),
                GetOutput::same_type(),
            )
            .with_fmt("str.base64_encode")
            .into()
    }

    pub fn encode(&self, encoding: &str) -> JsExpr {
        match encoding {
            "hex" => self.hex_encode(),
            "base64" => self.base64_encode(),
            _ => panic!("Encoding {} not supported", encoding),
        }
    }
}
