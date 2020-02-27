extern crate wasmi;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

fn main() -> Result<(), Box<Error>> {
    // load webassembly module from add.wasm into a vector of bytes, and create a Module from that buffer
    let mut buffer = Vec::new();
    {
        let mut f = File::open("../../Champ.1/add.wasm")?; 
        f.read_to_end(&mut buffer)?;
    }
    // Instantiate the module (it is a runnning copy)
    let module = wasmi::Module::from_buffer(buffer)?;
    // creates a new module with default set of imports
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("Failed to instantiate WASM module") // another way of forcing a panic if we get a failing result, except assert
        .assert_no_start(); // gives us an executable module instance that will panic if the module has a start function
    
    let mut args = Vec::<RuntimeValue>::new();
    args.push(RuntimeValue::from(42));  // convert Rust data to WASM's
    args.push(RuntimeValue::from(1));

    let result: Option<RuntimeValue> =
        instance.invoke_export("add", &args, &mut NopExternals)?;   // call exported func "add" of add.wasm 
    match result {
        Some(RuntimeValue::I32(v)) => {
            println!("The answer to your addition was {}", v);
        }
        Some(_) => {
            println!("Got a value of an unexpected data type");
        }
        None => {
            println!("Failed to get a result from wasm invocation");
        }
    }
    Ok(())
}