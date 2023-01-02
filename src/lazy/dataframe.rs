use polars::prelude::*;
use wasm_bindgen::prelude::*;

use super::expr::JsExpr;
use crate::{dataframe::JsDataFrame, error::JsPolarsErr, JsResult};
#[wasm_bindgen(js_name = LazyFrame)]
#[repr(transparent)]
#[derive(Clone)]
pub struct JsLazyFrame {
    ldf: LazyFrame,
}

impl From<LazyFrame> for JsLazyFrame {
    fn from(ldf: LazyFrame) -> Self {
        JsLazyFrame { ldf }
    }
}

impl JsLazyFrame {
    fn get_schema(&self) -> JsResult<SchemaRef> {
        self.ldf.schema().map_err(|e| JsPolarsErr::from(e).into())
    }
}

#[wasm_bindgen(js_class=LazyFrame)]
impl JsLazyFrame {
    #[wasm_bindgen(getter)]
    pub fn columns(&self) -> JsResult<JsValue> {
        let cols: Vec<String> = self.get_schema()?.iter_names().cloned().collect();
        serde_wasm_bindgen::to_value(&cols).map_err(|e| JsPolarsErr::from(e).into())
    }

    /// Cache the result once the execution of the physical plan hits this node.
    pub fn cache(&self) -> JsLazyFrame {
        self.ldf.clone().cache().into()
    }

    pub fn clone(&self) -> JsLazyFrame {
        self.ldf.clone().into()
    }
    #[wasm_bindgen(js_name = "__collect_from_worker", skip_typescript)]
    pub fn collect_from_worker(&self) -> JsResult<JsDataFrame> {
        self.ldf
            .clone()
            .collect()
            .map_err(|e| JsPolarsErr::from(e).into())
            .map(|df| df.into())
    }

    /// A string representation of the optimized query plan.
    pub fn describe_optimized_plan(&self) -> JsResult<String> {
        let result = self
            .ldf
            .describe_optimized_plan()
            .map_err(JsPolarsErr::from)?;
        Ok(result)
    }

    /// A string representation of the unoptimized query plan.
    pub fn describe_plan(&self) -> String {
        self.ldf.describe_plan()
    }

    /// Remove one or multiple columns from a DataFrame.
    pub fn drop(&self, _cols: JsValue) -> Self {
        // let ldf = self.ldf.clone();
        todo!()
        // ldf.drop_columns(cols).into()
    }

    /// Drop rows with null values from this DataFrame.
    /// This method only drops nulls row-wise if any single value of the row is null.
    pub fn drop_nulls(&self, cols: JsValue) -> JsLazyFrame {
        if cols.is_null() | cols.is_undefined() {
            return self.ldf.clone().drop_nulls(None).into();
        } else if cols.is_string() {
            return self
                .ldf
                .clone()
                .drop_nulls(Some(vec![col(&cols.as_string().unwrap())]))
                .into();
        } else {
            let cols: Vec<String> = serde_wasm_bindgen::from_value(cols).unwrap();
            let cols: Vec<Expr> = cols.iter().map(|name| col(&name)).collect();
            return self.ldf.clone().drop_nulls(Some(cols)).into();
        }
    }
    pub fn sort(&self, by_column: &str, reverse: bool, nulls_last: bool) -> JsLazyFrame {
        let ldf = self.ldf.clone();
        ldf.sort(
            by_column,
            SortOptions {
                descending: reverse,
                nulls_last,
            },
        )
        .into()
    }

    pub fn sort_by_exprs(
        &self,
        by_column: &js_sys::Array,
        reverse: JsValue,
        nulls_last: bool,
    ) -> JsResult<JsLazyFrame> {
        let ldf = self.ldf.clone();

        let reverse: Vec<bool> =
            serde_wasm_bindgen::from_value(reverse).map_err(JsPolarsErr::from)?;
        let exprs = js_exprs_to_exprs(by_column)?;
        Ok(ldf.sort_by_exprs(exprs, reverse, nulls_last).into())
    }

    pub fn join(
        &self,
        other: &JsLazyFrame,
        left_on: &js_sys::Array,
        right_on: &js_sys::Array,
        how: &str,
        suffix: &str,
        allow_parallel: bool,
        force_parallel: bool,
    ) -> JsResult<JsLazyFrame> {
        let ldf = self.ldf.clone();
        let other = other.ldf.clone();
        let left_on = js_exprs_to_exprs(left_on)?;
        let right_on = js_exprs_to_exprs(right_on)?;
        let how = match how {
            "left" => JoinType::Left,
            "inner" => JoinType::Inner,
            "outer" => JoinType::Outer,
            "semi" => JoinType::Semi,
            "anti" => JoinType::Anti,
            _ => {
                return Err(JsPolarsErr::Other(
                    "how should be one of inner, left, right, outer".into(),
                )
                .into())
            }
        };
        Ok(ldf
            .join_builder()
            .with(other)
            .left_on(left_on)
            .right_on(right_on)
            .allow_parallel(allow_parallel)
            .force_parallel(force_parallel)
            .how(how)
            .suffix(suffix)
            .finish()
            .into())
    }

    pub fn with_column(&mut self, expr: JsExpr) -> JsLazyFrame {
        let ldf = self.ldf.clone();
        ldf.with_column(expr.inner).into()
    }

    pub fn with_columns(&mut self, exprs: &js_sys::Array) -> JsResult<JsLazyFrame> {
        let ldf = self.ldf.clone();
        Ok(ldf.with_columns(js_exprs_to_exprs(exprs)?).into())
    }

    /// Explode lists to long format.
    pub fn explode(&self, _cols: &JsValue) -> JsLazyFrame {
        todo!()
    }

    /// Filter the rows in the DataFrame based on a predicate expression.
    /// @param predicate - Expression that evaluates to a boolean Series.
    /// @example
    /// ```js
    /// > lf = pl.DataFrame({
    /// ...   "foo": [1, 2, 3],
    /// ...   "bar": [6, 7, 8],
    /// ...   "ham": ['a', 'b', 'c']
    /// ... }).lazy()
    ///
    /// // Filter on one condition
    /// > await lf.filter(pl.col("foo").lt(3)).collect()
    /// shape: (2, 3)
    /// ┌─────┬─────┬─────┐
    /// │ foo ┆ bar ┆ ham │
    /// │ --- ┆ --- ┆ --- │
    /// │ i64 ┆ i64 ┆ str │
    /// ╞═════╪═════╪═════╡
    /// │ 1   ┆ 6   ┆ a   │
    /// ├╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┤
    /// │ 2   ┆ 7   ┆ b   │
    /// └─────┴─────┴─────┘
    /// ```
    pub fn filter(&self, expr: &JsExpr) -> JsResult<JsLazyFrame> {
        let ldf = self.ldf.clone();
        Ok(ldf.filter(expr.inner.clone()).into())
    }
    pub fn select(&self, exprs: &js_sys::Array) -> JsResult<JsLazyFrame> {
        let exprs = js_exprs_to_exprs(exprs)?;
        let ldf = self.ldf.clone();
        Ok(ldf.select(&exprs).into())
    }
}

pub(crate) fn js_exprs_to_exprs(iter: &js_sys::Array) -> JsResult<Box<[Expr]>> {
    use wasm_bindgen::convert::FromWasmAbi;
    use wasm_bindgen::JsCast;
    let iterator = js_sys::try_iter(iter)?.ok_or_else(|| "need to pass iterable JS values!")?;

    iterator
        .into_iter()
        .map(|jsv| {
            let jsv = jsv?;
            let key = JsValue::from_str("ptr");
            let ptr = js_sys::Reflect::get(&jsv, &key)?;
            let n: f64 = js_sys::Number::unchecked_from_js(ptr).into();
            let expr: JsExpr = unsafe { JsExpr::from_abi(n as u32) };
            Ok(expr.inner)
        })
        .collect()
}
