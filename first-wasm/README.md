From hello-wasm directory:

Install wasm-pack in case don't have it
```
$ cargo install wasm-pack
```

Install node in case don't have it.

Generate wasm file
```
$ wasm-pack build
```

Generate the npm package from the wasm file:
```
$ cd pkg
$ npm link
```

Move to site directory and link the npm package:
```
$ cd ../..
$ mkdir site
$ cd site
$ npm link hello-wasm
```

Update packages and run server:
```
$ npm install
$ npm run serve
```