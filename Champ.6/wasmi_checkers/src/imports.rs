use wasmi::{
    Error as InterpreterError, FuncInstance,
    FuncRef, ModuleImportResolver, Signature, ValueType,
};

pub const PIECEMOVED_INDEX: usize = 0;
pub const PIECECROWNED_INDEX: usize = 1;

pub struct RuntimeModuleImportResolver;

impl RuntimeModuleImportResolver {
    pub fn new() -> RuntimeModuleImportResolver {
        RuntimeModuleImportResolver {}
    }
}

impl<'a> ModuleImportResolver for RuntimeModuleImportResolver {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, InterpreterError> {
        let func_ref = match field_name {
            "piecemoved" => FuncInstance::alloc_host( // (1) Provide a FuncRef for the piecemoved function
                Signature::new(
                    &[
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                    ][..],
                    None,
                ),
                PIECEMOVED_INDEX,
            ),
            "piececrowned" => FuncInstance::alloc_host( // (2) Provide a FuncRef for the piececrowned function
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                PIECECROWNED_INDEX,
            ),
            _ => {
                return Err(InterpreterError::Function(format!( // (3) Return error if the module is trying to import a function we dont know about
                    "host module doesn't export function with name {}",
                    field_name
                )))
            }
        };
        Ok(func_ref)
    }
}