pub mod conversion;

pub mod string;

pub use conversion::*;
use polars::lazy::dsl;
use polars::prelude::*;
use wasm_bindgen::prelude::*;

pub use self::string::JsStringNameSpace;

#[wasm_bindgen(js_name=Expr)]
#[repr(transparent)]
#[derive(Clone)]
pub struct JsExpr {
    pub(crate) inner: Expr,
}

#[wasm_bindgen(js_class=Expr)]
impl JsExpr {
    #[wasm_bindgen(getter)]
    pub fn str(&self) -> JsStringNameSpace {
        JsStringNameSpace {
            inner: self.inner.clone(),
        }
    }

    /// Take absolute values
    pub fn abs(&self) -> JsExpr {
        self.clone().inner.abs().into()
    }

    /// Get the group indexes of the group by operation.
    /// Should be used in aggregation context only.
    /// @example
    /// ```js
    /// > df = pl.DataFrame({
    /// ...  "group": [
    /// ...      "one",
    /// ...      "one",
    /// ...      "one",
    /// ...      "two",
    /// ...      "two",
    /// ...      "two",
    /// ...  ],
    /// ...  "value": [94, 95, 96, 97, 97, 99],
    /// ... })
    /// >>> df.groupby("group", {maintain_order:True}).agg(pl.col("value").agg_groups())
    /// shape: (2, 2)
    /// ┌───────┬───────────┐
    /// │ group ┆ value     │
    /// │ ---   ┆ ---       │
    /// │ str   ┆ list[u32] │
    /// ╞═══════╪═══════════╡
    /// │ one   ┆ [0, 1, 2] │
    /// ├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
    /// │ two   ┆ [3, 4, 5] │
    /// └───────┴───────────┘
    /// ```
    pub fn agg_groups(&self) -> JsExpr {
        self.clone().inner.agg_groups().into()
    }

    /// Rename the output of an expression.
    /// @param name - new name
    /// @example
    /// ```js
    /// > df = pl.DataFrame({
    /// ...   "a": [1, 2, 3],
    /// ...   "b": ["a", "b", None],
    /// ... })
    /// > df
    /// shape: (3, 2)
    /// ╭─────┬──────╮
    /// │ a   ┆ b    │
    /// │ --- ┆ ---  │
    /// │ i64 ┆ str  │
    /// ╞═════╪══════╡
    /// │ 1   ┆ "a"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 2   ┆ "b"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 3   ┆ null │
    /// ╰─────┴──────╯
    /// > df.select([
    /// ...   pl.col("a").alias("bar"),
    /// ...   pl.col("b").alias("foo"),
    /// ... ])
    /// shape: (3, 2)
    /// ╭─────┬──────╮
    /// │ bar ┆ foo  │
    /// │ --- ┆ ---  │
    /// │ i64 ┆ str  │
    /// ╞═════╪══════╡
    /// │ 1   ┆ "a"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 2   ┆ "b"  │
    /// ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    /// │ 3   ┆ null │
    /// ╰─────┴──────╯
    /// ```
    pub fn alias(&self, alias: &str) -> JsExpr {
        self.clone().inner.alias(alias).into()
    }
    /// Get the index of the maximal value.
    pub fn arg_max(&self) -> JsExpr {
        self.clone().inner.arg_max().into()
    }
    /// Get the index of the minimal value.
    pub fn arg_min(&self) -> JsExpr {
        self.clone().inner.arg_min().into()
    }
    /// Get the index values that would sort this column.
    /// @param reverse
    /// - false -> order from small to large.
    /// - true -> order from large to small.
    /// @returns UInt32 Series
    pub fn arg_sort(&self, reverse: bool) -> JsExpr {
        self.clone()
            .inner
            .arg_sort(SortOptions {
                descending: reverse,
                nulls_last: true,
            })
            .into()
    }

    /// Get index of first unique value.
    pub fn arg_unique(&self) -> JsExpr {
        self.clone().inner.arg_unique().into()
    }

    /// Fill missing values with the next to be seen values
    pub fn backward_fill(&self) -> JsExpr {
        self.clone().inner.backward_fill(None).into()
    }

    /// Cast between data types.
    pub fn cast() -> JsExpr {
        todo!()
    }

    /// Count the number of values in this expression
    pub fn count(&self) -> JsExpr {
        self.clone().inner.count().into()
    }

    /// Calculate the n-th discrete difference.
    /// @param n - number of slots to shift
    /// @param nullBehavior -  'ignore' or 'drop'
    pub fn diff(&self) -> JsExpr {
        todo!()
    }

    /// Compute the dot/inner product between two Expressions
    /// @param other Expression to compute dot product with
    pub fn dot(&self, _other: &JsExpr) -> JsExpr {
        todo!()
        // self.inner.clone().dot(other.inner.clone()).into()
    }

    ///  Exclude certain columns from a wildcard/regex selection.
    ///
    ///  You may also use regexes in the exclude list. They must start with `^` and end with `$`.
    ///
    ///  @param columns Column(s) to exclude from selection
    ///  @example
    ///  ```js
    ///   > df = pl.DataFrame({
    ///   ...   "a": [1, 2, 3],
    ///   ...   "b": ["a", "b", None],
    ///   ...   "c": [None, 2, 1],
    ///   ...})
    ///   > df
    ///   shape: (3, 3)
    ///   ╭─────┬──────┬──────╮
    ///   │ a   ┆ b    ┆ c    │
    ///   │ --- ┆ ---  ┆ ---  │
    ///   │ i64 ┆ str  ┆ i64  │
    ///   ╞═════╪══════╪══════╡
    ///   │ 1   ┆ "a"  ┆ null │
    ///   ├╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///   │ 2   ┆ "b"  ┆ 2    │
    ///   ├╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///   │ 3   ┆ null ┆ 1    │
    ///   ╰─────┴──────┴──────╯
    ///   > df.select(
    ///   ...   pl.col("*").exclude("b"),
    ///   ... )
    ///  shape: (3, 2)
    ///  ╭─────┬──────╮
    ///  │ a   ┆ c    │
    ///  │ --- ┆ ---  │
    ///  │ i64 ┆ i64  │
    ///  ╞═════╪══════╡
    ///  │ 1   ┆ null │
    ///  ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///  │ 2   ┆ 2    │
    ///  ├╌╌╌╌╌┼╌╌╌╌╌╌┤
    ///  │ 3   ┆ 1    │
    ///  ╰─────┴──────╯
    ///  ```
    /// /
    pub fn exclude(&self) -> JsExpr {
        todo!()
    }

    /// Explode a list or utf8 Series.
    /// This means that every item is expanded to a new row.
    pub fn explode(&self) -> JsExpr {
        self.clone().inner.explode().into()
    }
    pub fn extend(&self) -> JsExpr {
        todo!()
    }
    pub fn extend_constant(&self) -> JsExpr {
        todo!()
    }

    /// Fill nan value with a fill value
    pub fn fill_nan(&self, expr: &JsExpr) -> JsExpr {
        self.inner.clone().fill_nan(expr.inner.clone()).into()
    }
    /// Fill null value with a fill value or strategy
    pub fn fill_null(&self, expr: &JsExpr) -> JsExpr {
        self.clone().inner.fill_null(expr.inner.clone()).into()
    }

    /// Filter a single column.
    /// Mostly useful in in aggregation context.
    /// If you want to filter on a DataFrame level, use `LazyFrame.filter`.
    /// @param predicate -  Boolean expression.
    pub fn filter(&self, predicate: &JsExpr) -> JsExpr {
        self.clone().inner.filter(predicate.inner.clone()).into()
    }

    /// Get the first value.
    pub fn first(&self) -> JsExpr {
        self.clone().inner.first().into()
    }

    /// Fill missing values with the latest seen values
    pub fn forward_fill(&self) -> JsExpr {
        self.clone().inner.forward_fill(None).into()
    }
}

#[wasm_bindgen(js_class=Expr)]
impl JsExpr {
    pub fn add(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Plus, rhs.inner.clone()).into()
    }

    pub fn sub(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Minus, rhs.inner.clone()).into()
    }

    pub fn mul(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Multiply, rhs.inner.clone()).into()
    }

    pub fn truediv(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::TrueDivide, rhs.inner.clone()).into()
    }

    pub fn modulus(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Modulus, rhs.inner.clone()).into()
    }

    pub fn floordiv(&self, rhs: &JsExpr) -> JsExpr {
        dsl::binary_expr(self.inner.clone(), Operator::Divide, rhs.inner.clone()).into()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.inner)
    }

    pub fn eq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.eq(other.inner.clone()).into()
    }

    pub fn neq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.neq(other.inner.clone()).into()
    }

    pub fn gt(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.gt(other.inner.clone()).into()
    }

    pub fn gt_eq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.gt_eq(other.inner.clone()).into()
    }

    pub fn lt_eq(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.lt_eq(other.inner.clone()).into()
    }

    pub fn lt(&self, other: &JsExpr) -> JsExpr {
        self.clone().inner.lt(other.inner.clone()).into()
    }

    pub fn is_not(&self) -> JsExpr {
        self.clone().inner.not().into()
    }

    pub fn is_null(&self) -> JsExpr {
        self.clone().inner.is_null().into()
    }

    pub fn is_not_null(&self) -> JsExpr {
        self.clone().inner.is_not_null().into()
    }

    pub fn is_infinite(&self) -> JsExpr {
        self.clone().inner.is_infinite().into()
    }

    pub fn is_finite(&self) -> JsExpr {
        self.clone().inner.is_finite().into()
    }

    pub fn is_nan(&self) -> JsExpr {
        self.clone().inner.is_nan().into()
    }

    pub fn is_not_nan(&self) -> JsExpr {
        self.clone().inner.is_not_nan().into()
    }

    pub fn min(&self) -> JsExpr {
        self.clone().inner.min().into()
    }

    pub fn max(&self) -> JsExpr {
        self.clone().inner.max().into()
    }

    pub fn mean(&self) -> JsExpr {
        self.clone().inner.mean().into()
    }

    pub fn median(&self) -> JsExpr {
        self.clone().inner.median().into()
    }

    pub fn sum(&self) -> JsExpr {
        self.clone().inner.sum().into()
    }

    pub fn n_unique(&self) -> JsExpr {
        self.clone().inner.n_unique().into()
    }

    pub fn unique(&self) -> JsExpr {
        self.clone().inner.unique().into()
    }

    pub fn unique_stable(&self) -> JsExpr {
        self.clone().inner.unique_stable().into()
    }

    pub fn last(&self) -> JsExpr {
        self.clone().inner.last().into()
    }

    pub fn list(&self) -> JsExpr {
        self.clone().inner.list().into()
    }

    pub fn quantile(&self, _quantile: &JsExpr) -> JsExpr {
        todo!()
    }

    pub fn value_counts(&self, multithreaded: bool, sorted: bool) -> JsExpr {
        self.inner
            .clone()
            .value_counts(multithreaded, sorted)
            .into()
    }

    pub fn unique_counts(&self) -> JsExpr {
        self.inner.clone().unique_counts().into()
    }

    pub fn sort_with(&self, descending: bool, nulls_last: bool) -> JsExpr {
        self.clone()
            .inner
            .sort_with(SortOptions {
                descending,
                nulls_last,
            })
            .into()
    }

    pub fn take(&self, idx: &JsExpr) -> JsExpr {
        self.clone().inner.take(idx.inner.clone()).into()
    }
    pub fn sort_by() -> JsExpr {
        todo!()
    }

    pub fn shift(&self, periods: i64) -> JsExpr {
        self.clone().inner.shift(periods).into()
    }

    pub fn shift_and_fill(&self, periods: i64, fill_value: &JsExpr) -> JsExpr {
        self.clone()
            .inner
            .shift_and_fill(periods, fill_value.inner.clone())
            .into()
    }

    pub fn fill_null_with_strategy(&self) -> JsExpr {
        todo!()
        // self.inner
        //     .clone()
        //     .apply(move |s| s.fill_null(strategy.0), GetOutput::same_type())
        //     .with_fmt("fill_null")
        //     .into()
    }

    pub fn drop_nulls(&self) -> JsExpr {
        self.inner.clone().drop_nulls().into()
    }

    pub fn drop_nans(&self) -> JsExpr {
        self.inner.clone().drop_nans().into()
    }

    pub fn reverse(&self) -> JsExpr {
        self.clone().inner.reverse().into()
    }

    pub fn std(&self, ddof: Option<u8>) -> JsExpr {
        let ddof = ddof.unwrap_or(1);
        self.clone().inner.std(ddof).into()
    }

    pub fn var(&self, ddof: Option<u8>) -> JsExpr {
        let ddof = ddof.unwrap_or(1);
        self.clone().inner.var(ddof).into()
    }
    pub fn is_unique(&self) -> JsExpr {
        self.clone().inner.is_unique().into()
    }

    pub fn is_first(&self) -> JsExpr {
        self.clone().inner.is_first().into()
    }

    pub fn take_every(&self, n: i64) -> JsExpr {
        self.clone()
            .inner
            .map(
                move |s: Series| Ok(s.take_every(n as usize)),
                GetOutput::same_type(),
            )
            .with_fmt("take_every")
            .into()
    }
    
    pub fn tail(&self, n: Option<i64>) -> JsExpr {
        let n = n.map(|v| v as usize);
        self.clone().inner.tail(n).into()
    }

    pub fn head(&self, n: Option<i64>) -> JsExpr {
        let n = n.map(|v| v as usize);
        self.clone().inner.head(n).into()
    }
    pub fn slice(&self, offset: &JsExpr, length: &JsExpr) -> JsExpr {
        self.inner
            .clone()
            .slice(offset.inner.clone(), length.inner.clone())
            .into()
    }
    pub fn round(&self, decimals: u32) -> JsExpr {
        self.clone().inner.round(decimals).into()
    }

    pub fn floor(&self) -> JsExpr {
        self.clone().inner.floor().into()
    }

    pub fn ceil(&self) -> JsExpr {
        self.clone().inner.ceil().into()
    }
    pub fn clip(&self) -> JsExpr {
        todo!()
    }

    pub fn is_duplicated(&self) -> JsExpr {
        self.clone().inner.is_duplicated().into()
    }
}
