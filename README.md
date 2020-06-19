# Example of using Rust & WebAssembly to build an Fn Function

This example uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) to compile the Rust code in `/src/lib.rs` to WebAssembly and build an NPM package.

The NPM package is then published to npmjs (it can be downloaded as `@crush-157/rust-fn`).

The package is then referenced in the `package.json` and `func.js` of the function which otherwise uses the Node.js FDK and runtime in the usual way.
