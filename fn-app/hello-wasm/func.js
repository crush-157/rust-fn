const fdk=require('@fnproject/fdk');
const wasm=require('@crush-157/rust-fn');

fdk.handle(function(input){
  return wasm.hello_json(input);
})
