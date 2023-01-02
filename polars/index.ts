import { DataFrame } from "./dataframe.js";
import { Series } from "./series/index.js";
import * as pli from "./core/browser.js";
import { waitForMsgType } from "./utils.js";
import { LazyFrame } from "./lazy/frame/index.js";

const wasm = await pli.default();
const POLARS_WORKER = new Worker(new URL("./worker.js", import.meta.url), {
  type: "module",
});

POLARS_WORKER.postMessage({ type: "start", payload: wasm.memory });

await waitForMsgType(POLARS_WORKER, "ready");

export * from "./io.js";
export * from "./lazy/index.js";
export { DataFrame, Series, LazyFrame, POLARS_WORKER };
