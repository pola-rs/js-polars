import { DataFrame, POLARS_WORKER } from "./index.js";
import { waitForMsgType } from "./utils.js";

const readCsvDefaultOptions = {
  inferSchemaLength: 100,
  hasHeader: true,
  ignoreErrors: true,
  chunkSize: 10000,
  skipRows: 0,
  sep: ",",
  rechunk: false,
  encoding: "utf8",
  lowMemory: false,
  parseDates: false,
  skipRowsAfterHeader: 0,
};

export interface ReadCsvOptions {
  inferSchemaLength?: number;
  hasHeader?: boolean;
  ignoreErrors?: boolean;
  chunkSize?: number;
  skipRows?: number;
  sep?: string;
  rechunk?: boolean;
  encoding?: string;
  lowMemory?: boolean;
  parseDates?: boolean;
  skipRowsAfterHeader?: number;
  numRows?: number;
  numThreads?: number;
}

export async function read_csv(
  path: string | Uint8Array,
  options: ReadCsvOptions = readCsvDefaultOptions,
): Promise<DataFrame> {
  let buf: Uint8Array;
  if (typeof path === "string") {
    try {
      const url = new URL(path);
      if (url.protocol === "http:" || url.protocol === "https:") {
        const response = await fetch(path);
        let arrayBuffer = await response.arrayBuffer();
        buf = new Uint8Array(arrayBuffer);
      } else {
        throw new Error("Only http and https protocols are supported");
      }
    } catch (e) {
      throw new Error(`${path} is not a valid url`);
    }
  } else {
    buf = path;
  }

  POLARS_WORKER.postMessage(
    {
      type: "read_csv",
      options,
      buf,
    },
    [buf.buffer],
  );
  const event: any = await waitForMsgType(POLARS_WORKER, "read_csv");
  const ptr = event.data.ptr;

  return DataFrame.wrap(ptr);
}
