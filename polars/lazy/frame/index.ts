import { DataFrame, POLARS_WORKER } from "../../index.js";
import * as pli from "../../core/browser.js";
import { waitForMsgType } from "../../utils.js";

export class LazyFrame extends pli.LazyFrame {
  private ptr!: number;

  static wrap(ptr: number): LazyFrame {
    const obj = Object.create(LazyFrame.prototype);
    obj.ptr = ptr;

    return obj;
  }

  static __wrap_ptr(lf: pli.LazyFrame): LazyFrame {
    return LazyFrame.wrap((lf as any).ptr);
  }

  async collect(): Promise<DataFrame> {
    const ptr = this.ptr;
    POLARS_WORKER.postMessage({
      type: "LazyFrame::collect",
      ptr,
    });
    const event: any = await waitForMsgType(
      POLARS_WORKER,
      "LazyFrame::collect",
    );
    const df_ptr = event.data.ptr;
    return DataFrame.wrap(df_ptr);
  }

  override select(exprs: pli.Expr[]): LazyFrame {
    return LazyFrame.__wrap_ptr(super.select(exprs));
  }
  override filter(expr: pli.Expr): LazyFrame {
    return LazyFrame.__wrap_ptr(super.filter(expr));
  }
  override drop_nulls(subset?: string | string[]): LazyFrame {
    return LazyFrame.__wrap_ptr(super.drop_nulls(subset));
  }
}
