# Example of using Rust & WebAssembly to build an Fn Function

This example shows how to:
- Compile Rust code to [WebAssembly](https://webassembly.org/) "wasm".
- Create and publish an NPM package containing the generated "wasm" code.
- Build a function using the Node.js FDK that includes the package and calls the Rust methods.

## Prerequisites
### Cloud Shell

The preferred approach for working with the [Oracle Functions](https://www.oracle.com/cloud/cloud-native/functions/) service is to use [Oracle Cloud Shell](https://docs.cloud.oracle.com/en-us/iaas/Content/API/Concepts/cloudshellintro.htm) to develop, deploy and run your functions.

To use the Cloud Shell, carry out the following steps of the [Oracle Functions Quick Start Guide for Cloud Shell](https://www.oracle.com/webfolder/technetwork/tutorials/infographics/oci_functions_cloudshell_quickview/functions_quickview_top/functions_quickview/index.html):
- A. Set up your tenancy
- B. Create your application
- C. Set up your Cloud Shell dev environment

### Non Cloud Shell
If you are *not* using Cloud Shell, then you will need to:
- [Install the Fn CLI](https://fnproject.io/tutorials/install/)
- Configure your _context_ for the CLI.  Follow the appropriate instructions for:
    - [Oracle Functions](https://docs.cloud.oracle.com/en-us/iaas/Content/Functions/Tasks/functionscreatefncontext.htm)
    - [Local Fn Environment]()
- [Install Node.js](https://nodejs.org/en/download/)

### Development Tools
- [Install Rust](https://www.rust-lang.org/tools/install)
- Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/):

`cargo install wasm-pack`

### npm.js account
[Sign up](https://www.npmjs.com/signup) for an npm.js account if you don't have one already.

## Rust to WebAssembly
### Use `wasm-pack` to create a WebAssembly package
Run the following command:
`$ wasm-pack new rust-fn`

This creates a new directory `./rust-fn` containing a skeleton Rust/wasm package.

### Add serialisation support
To be able to serialise and deserialise objects in Rust we need to add serialisation support.

In the `./rust-fn` directory there is a `Cargo.toml` file. 

Modify this so that the `[dependencies]` section matches the example:
```
[package]
name = "rust-fn"
version = "0.1.0"
authors = ["Ewan Slater <ewan.slater@gmail.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = { version = "0.2.63", features = ["serde-serialize"] }
serde = { version = "1.*", features = ["derive"] }

# The `console_eror_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.6", optional = true }

# `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
# compared to the default allocator's ~10K. It is slower than the default
# allocator, however.
#
# Unfortunately, `wee_alloc` requires nightly Rust when targeting wasm for now.
wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.13"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

### Export Rust Functions

The boilerplate Rust code is in `./rust-fn/lib.rs`.

You will add your functions to this file.

Functions that you want to be able to call from JavaScript need to be exported by WebAssembly.  These functions need to have the annotation `#[wasm_bindgen]` applied.

Types that you want to be able to serialize will need to have the annotation `#[derive(Serialize)]`.

Types that you want to be able to deserialize will need to have the annotation `#[derive(Deserialize)]`.

Edit the `lib.rs` file so that it contains the following code:
```
mod utils;

use wasm_bindgen::prelude::*;
use serde::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen (js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello_string(s: &str) -> String {
    format!("Hello, {}!", s)
}

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("Hello, rust-fn!")
}

#[derive(Deserialize)]
pub struct Name {
    pub name: String
}

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[wasm_bindgen]
pub fn greet_json() -> JsValue {
    let msg_string = String::from("Hello, rust-fn!");
    let msg = Message {
        message: msg_string
    };
    JsValue::from_serde(&msg).unwrap()
}

#[wasm_bindgen]
pub fn hello_json(i: &JsValue) -> JsValue {
    let i_name: Name = i.into_serde()
        .unwrap_or_else( |_e| {
            Name { name: "World".to_string() }
        });
    let msg = Message {
        message: format!("Hello, {}!", i_name.name)
    };
    JsValue::from_serde(&msg).unwrap()
}
```

Note that there are two sets of functions which are equivalent but differ in argument and return types:
- String:
    - `greet()` and `hello_string()` return a `String` value.
    - `hello_string()` takes a `String` as input.
- JSON:
    - `greet_json()` and `hello_json()` return a JSON value.
    - `hello_json()` takes a JSON value as input.

### Compile the Package
Run the command:

`$ wasm-pack build --target nodejs --scope <your-npm.js-user-id>`

This will create a package in the `./pkg` directory that can be used from node and published to npm.

### Publish the Package
From the `./pkg` directory, run the command:

`$ npm publish --access public`

This will publish the NPM to npm.js (it can be downloaded as `@<your-npm.js-user-id>/rust-fn`).

If you wish, you can use `npm install` to install your newly published package and test it using the `node` REPL from the command line.

## Create new function
You can now create a new (Oracle Functions / Fn) function using the Node FDK:

`$ fn init --runtime node <function-name>`

Edit the `package.json` file to include your package:
```
{
  "name": "hellofn",
  "version": "1.0.0",
  "description": "example function",
  "main": "func.js",
  "author": "",
  "license": "Apache-2.0",
  "dependencies": {
    "@fnproject/fdk": ">=0.0.18",
    "@<your-npm.js-user-id>/rust-fn": ">=0.1.0"
  }
}
```

You can then reference the package in the JavaScript code of the function (`func.js`) of the Node.js function in the usual way.

## Sample functions
There are two sample functions:
- `busy-rust`
- `lazy-rust`

These reflect whether all the work, including the JSON parsing, is taking place in Rust, or whether the JSON is being parsed in the JavaScript part of the function.

Parsing JSON in JavaScript is a bit easier than in Rust, but it's arguably more elegant to do all the work in Rust.

Both styles are included here for completeness.

### busy-rust
The JSON payload is passed to the Rust function which parses and processes it.
```
const fdk=require('@fnproject/fdk');
const wasm=require('@crush-157/rust-fn');

fdk.handle(function(input){
  return wasm.hello_json(input);
})
```

### lazy-rust
The JSON payload is parsed in JavaScript, and the relevant values passed as arguments to the Rust function. 

```
const fdk=require('@fnproject/fdk');
const wasm=require('@crush-157/rust-fn');

fdk.handle(function(input){
  let name = 'World';
  if (input.name) {
    name = input.name;
  }
  return {'message': wasm.hello_string(name) }
})
```