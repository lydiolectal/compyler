var importObject = {
  imports: {
    print: arg => console.log(arg),
  },
};

async function load() {
  console.log('fetching main.wasm...');
  let response = await fetch('main_js.wasm');

  console.log('getting wasm bytes...');
  let bytes = await response.arrayBuffer();

  console.log('compiling webassembly module...');
  let module = await WebAssembly.compile(bytes);

  console.log('constructing webassembly instance...');
  let instance = new WebAssembly.Instance(module, importObject);

  console.log('running webassembly main...');
  instance.exports.main();
}

console.log('calling load...');
load();
