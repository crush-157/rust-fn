let wasm = require('./pkg/rust_fn');
console.log(wasm.greet());
console.log(wasm.hello_string("Nikita"));
