# Example of using Rust & WebAssembly to build an Fn Function

## Overview

This example uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) to compile the Rust code in `/src/lib.rs` to WebAssembly and build an NPM package.

The NPM package is then published to npmjs (it can be downloaded as `@crush-157/rust-fn`).

The package is then referenced in the `package.json` and `func.js` of a function which otherwise uses the Node.js FDK and runtime in the usual way.

There are two example functions:
- `busy-rust` - the JSON payload is passed to the Rust function which parses and processes it.
- `lazy-rust` - the JSON payload is parsed in JavaScript, and the relevant values passed as arguments to the Rust function. 

The Rust `lib.rs` file therefore contains two sets of functions, reflecting the different styles.

The functions with the `_json` suffix are for the `busy-rust` case where Rust is responsible for parsing the JSON.
 
All of the functions can be run and tested with `node` from the command line.
