## Walk thougn the Current Dir by Rust crate(ignore)

### Why

- f.f.f.f..f.f..ff...f.ff.f..fast

### Use

```js
const files = require('find-files-rust').find('./');
```

### Demo

```
npm run start
```

see Result

```bash
[ './',
  './Cargo.toml',
  './cli-demo.js',
  './index.js',
  './readme.md',
  './yarn.lock',
  './package-lock.json',
  './package.json',
  './rust-dylib',
  './rust-dylib/libfind_files.dylib',
  './src',
  './src/lib.rs' ]
cost time: 0.009000000000000008 ms
```

### Build

- Rust, [lib.rs](src/lib.rs) with `libc`

```
cargo build --release
```

cp dylib into the dir named in `package.json` and `index.js`

```
cp target/release/libfind_files.dylib ./rust-dylib
```

- Node, [index.js](index.js) with `node-ffi`

```js
var lib = ffi.Library(path.join(__dirname, './rust-dylib/libfind_files'), {
  find_files: ['char *', ['string']],
  free_memory: ['void', ['char *']]
});
```

- [more details of `rust-ffi` ](https://github.com/shepmaster/rust-ffi-omnibus)

## Use by

- [doc-templite](https://github.com/chinanf-boy/doc-templite)
- [npm-modules-size](https://github.com/chinanf-boy/npm-modules-size)
