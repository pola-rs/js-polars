# @pola-rs/browser
This package contains the js <--> WASM bindings for the polars library.

# Warning
This package is highly experimental and is not meant for production usage. It is provided as-is and may contain bugs or incomplete features. Use at your own risk. Do not rely on this package for critical applications.

We make no guarantees about the stability, reliability, or performance of this package. It may undergo significant changes or be removed at any time.


## Startup overhead

Please be aware that this package has significant startup overhead when run in the browser, due in part to the size of the WASM binary, as well as spawning of workers & threadpools. 

We recommend only using this package in cases where the benefits of the features it provides outweigh the added startup overhead.

## Example usage

install via npm
```
 npm i -s @pola-rs/browser
```
install via yarn
```
yarn add @pola-rs/browser
```

Usage

```js

import * as pl from "@pola-rs/browser"
const filepath = "https://raw.githubusercontent.com/pola-rs/polars/master/examples/datasets/foods2.csv"

let df = await pl.read_csv(filepath)

let lf = df.lazy();
df = await lf.collect()

console.table(df.head(10).to_records())
```