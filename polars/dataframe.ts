import * as pli from "./core/browser.js";
import { LazyFrame } from "./index.js";

export class DataFrame extends pli.DataFrame {
  private ptr!: number;

  static wrap(ptr: number): DataFrame {
    const obj: DataFrame = Object.create(DataFrame.prototype);
    obj.ptr = ptr;

    return obj;
  }

  static __wrap_ptr(df: pli.DataFrame): DataFrame {
    return DataFrame.wrap((df as any).ptr);
  }

  override lazy(): LazyFrame {
    return LazyFrame.__wrap_ptr(super.lazy());
  }
}
