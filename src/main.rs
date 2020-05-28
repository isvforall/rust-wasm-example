use std::fs::File;
use std::io::prelude::Write;
use wabt::wat2wasm;
use wasmtime::{Instance, Module, Store};

fn main() {
    let wat = r#"
    (module
    (func (export "square") (param $i i32) (result i32)
        get_local $i
        get_local $i
        i32.mul
    )
    (func (export "double") (param $i i32) (result i32)
        get_local $i
        get_local $i
        i32.add
    ))"#;

    let store = Store::default();
    let module = Module::new(&store, wat).unwrap();
    let instance = Instance::new(&module, &[]).unwrap();

    let square_fn = instance.get_func("square").unwrap();
    let square = square_fn.get1::<i32, i32>().unwrap();

    let double_fn = instance.get_func("double").unwrap();
    let double = double_fn.get1::<i32, i32>().unwrap();

    println!("square function {:?}", square(5));
    println!("double function {:?}", double(5));

    // web
    let wasm_binary = wat2wasm(wat.as_bytes()).unwrap();
    let mut f = File::create("square_double.wasm").unwrap();
    f.write_all(&wasm_binary).unwrap();
}
