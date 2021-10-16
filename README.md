# rust-tuto

## Setup

- [Setup for Ubuntu 20.04](./SetupUbuntu.md)
- [Setup for Windows 11](./SetupWindows.md)

### Launch JupyterLab
```bash
# launch jupyter lab
## * port: 8888
## * token: empty
## * project path: ./
$ jupyter lab --port=8888 --ServerApp.token='' --project=.

## => http://localhost:8888/lab
```

***

## WebAssembly tutorial

https://rustwasm.github.io/book/game-of-life/introduction.html

### Environment
- Rust: `1.55.0`
    - cargo-generate: `0.10.3`
- Node.js: `16.10.0`
    - npm: `8.1.0`
    - wasm-pack: `0.10.1`

#### Setup
```bash
# install wasm-pack
$ npm i -g wasm-pack

# confirm wasm-pack version
$ wasm-pack -V

wasm-pack 0.10.1
```

### Create WASM project
```bash
# create WASM template project: `wasm-life-game` from https://rustwasm.github.io/book/game-of-life/introduction.html
$ cargo generate -n wasm-life-game --git https://github.com/rustwasm/wasm-pack-template

# set current directory to ./wasm-life-game/
$ cd ./wasm-life-game/
```

#### Structure
```bash
wasm-life-game/
|_ src/
|  |_ lib.rs   # root rust crate that will be compiled to WebAssembly
|  |_ utils.rs # common utilities to make working with rust compiled to WebAssembly easier
|
|_ tests/ # test code
|  :
|
|_ Cargo.toml  # cargo metadata file
```

#### ./src/lib.rs
```rust
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-life-game!");
}
```

### Build the project
```bash
$ wasm-pack build

## => wasm files will be compiled to ./pkg/
```

### Putting WebAssembly into a web page
```bash
# -- current directory: ./wasm-life-game/

# create wasm-app web project: `www`
$ npm init wasm-app www

$ cd www
```

#### ./www/package.json: add local dependent package `wasm-life-game`
```json
{
    // ...
    "dependencies": {
        "wasm-life-game": "file:../pkg" // add `wasm-life-game` package from ../pkg/
    }
}
```

#### ./www/index.js: import `wasm-life-game`
```javascript
import * as wasm from 'wasm-life-game';

wasm.greet();
```

#### install dependencies
```bash
# -- current directory: ./wasm-life-game/www/

# install dependencies from package.json
$ npm i
```

#### Serving local web application
```bash
# -- current directory: ./wasm-life-game/www/

# serve local web application
$ npm run start

## => http://localhost:8080/
### |> execute wasm.greet()
###    |_ wasm-life-game/src/lib.rs
###       |_ pub fn greet(): alert("Hello, wasm-life-game!")
```
