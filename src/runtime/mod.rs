pub mod environment;
pub(crate) mod eval;
pub mod interpreter;
pub mod values;

pub use environment::Environment;
pub use interpreter::evaluate;
pub use values::RuntimeValue;
pub(self) use values::{NullVal, NumberVal, ValueType};
