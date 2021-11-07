## 参考文献
https://github.com/forcia/rustbook/tree/master/ch06/6-2/mandelbrot

こちらのリポジトリ(と書籍)を参考に自力でwasmを動かせるようにしています。
## 🚴 Usage
### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🛠️ Run with `npm`
```
npm init wasm-app www
cd www
npm install
npm run start
```
http://localhost:8080/****


### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
