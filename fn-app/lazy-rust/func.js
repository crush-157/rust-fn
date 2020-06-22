const fdk=require('@fnproject/fdk');
const wasm=require('@crush-157/rust-fn');

fdk.handle(function(input){
  let name = 'World';
  if (input.name) {
    name = input.name;
  }
  return {'message': wasm.hello_string(name) }
})
