<div align="center">
  <h1><code>wasm-game-of-life</code></h1>
  <p><strong>Conway's Game of Life implemented using Rust and WebAssembly.</strong></p>
</div>

## Usage

### Build with `wasm-pack build`

```
wasm-pack build
```

### Run with `npm start`

```
cd www
npm install
npm start
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --chrome
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
