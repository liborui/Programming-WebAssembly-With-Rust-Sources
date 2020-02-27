use std::fmt;
use std::fs::File;
use wasmi::{
    Error as InterpreterError, Externals, FuncInstance, FuncRef,
    HostError, ImportsBuilder, Module, ModuleImportResolver, ModuleInstance,
    ModuleRef, RuntimeArgs, RuntimeValue, Signature, Trap, ValueType,
};

#[cfg(any(target_arch = "armv7", target_arch = "arm"))] // (1) Conditionally add Blinkt to the module’s scope
use blinkt::Blinkt;

fn load_module(path: &str) -> Module {
    use std::io::prelude::*;
    let mut file = File::open(path).unwrap();
    let mut wasm_buf = Vec::new();
    file.read_to_end(&mut wasm_buf).unwrap();
    Module::from_buffer(&wasm_buf).unwrap()
}

pub fn get_module_instance(path: &str) -> ModuleRef {
    let module = load_module(path);
    let mut imports = ImportsBuilder::new();
    imports.push_resolver("env", &RuntimeModuleImportResolver);

    ModuleInstance::new(&module, &imports)
        .expect("Failed to instantiate module")
        .assert_no_start()
}

pub const SENSOR_BATTERY: i32 = 20;


#[derive(Debug)]
pub enum Error {
    Interpreter(InterpreterError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<InterpreterError> for Error {
    fn from(e: InterpreterError) -> Self {
        Error::Interpreter(e)
    }
}

impl HostError for Error {}

pub struct Runtime {
    #[cfg(any(target_arch = "armv7", target_arch = "arm"))]
    blinkt: Blinkt, // (2) Conditionally add a blinkt field to the Runtime struct
    pub frame: i32,
    pub remaining_battery: f64,
}

impl Runtime {
    #[cfg(any(target_arch = "armv7", target_arch = "arm"))]
    pub fn new() -> Runtime {
        println!("Instiantiating WASM runtime (ARM)");
        Runtime {
            blinkt: Blinkt::new().unwrap(),
            frame: 0,
            remaining_battery: 100.0,
        }
    }

    #[cfg(not(any(target_arch = "armv7", target_arch = "arm")))]
    pub fn new() -> Runtime {
        println!("Instantiating WASM runtime (non-ARM)");
        Runtime {
            frame: 0,
            remaining_battery: 100.0,
        }
    }
}

impl Externals for Runtime {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {

        match index { // (3) The apply function will have an index of 0.
            0 => {
                let idx: i32 = args.nth(0);
                let red: i32 = args.nth(1);
                let green: i32 = args.nth(2);
                let blue: i32 = args.nth(3);
                self.set_led(idx, red, green, blue);
                Ok(None)
            }
            _ => panic!("Unknown function index!"),
        }
    }
}

impl Runtime {
    #[cfg(not(any(target_arch = "armv7", target_arch = "arm")))]
    fn set_led(&self, idx: i32, red: i32, green: i32, blue: i32) {
        println!("[LED {}]: {}, {}, {}", idx, red, green, blue);
    }

    #[cfg(any(target_arch = "armv7", target_arch = "arm"))]
    fn set_led(&mut self, idx: i32, red: i32, green: i32, blue: i32) {
        self.blinkt
            .set_pixel(idx as usize, red as u8, green as u8, blue as u8);
        self.blinkt.show().unwrap();
    }

    #[cfg(not(any(target_arch = "armv7", target_arch = "arm")))]
    pub fn shutdown(&mut self) {
        println!("WASM runtime shut down.");
        self.halt();
    }

    #[cfg(any(target_arch = "armv7", target_arch = "arm"))]
    pub fn shutdown(&mut self) {
        println!("WASM runtime shut down.");
        self.blinkt.clear();
        self.blinkt.cleanup().unwrap();
        self.halt();
    }

    fn halt(&self) {
        ::std::process::exit(0);
    }

    pub fn reduce_battery(&mut self) {
        self.remaining_battery -= 1.0;
        if self.remaining_battery < 0.0 {
            self.remaining_battery = 100.0;
        }
    }

    pub fn advance_frame(&mut self) {
        self.frame += 1;
        if self.frame > 1_000_000_000 {
            self.frame = 0;
        }
    }
}

struct RuntimeModuleImportResolver;

impl<'a> ModuleImportResolver for RuntimeModuleImportResolver {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, InterpreterError> {
        println!("Resolving {}", field_name);
        let func_ref = match field_name {
            "set_led" => FuncInstance::alloc_host( // (4) The set_led function is the only one exported by the host/imported by the module
                Signature::new(
                    &[
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                    ][..],
                    None,
                ),
                0,
            ),
            _ => {
                return Err(InterpreterError::Function(format!(
                    "host module doesn't export function with name {}",
                    field_name
                )))
            }
        };
        Ok(func_ref)
    }
}