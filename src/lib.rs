use pyo3::{prelude::*};
use cel_interpreter::{Context, Program, Value};
use pyo3::types::PyDict;

/* As I understood `unsandable` prevents class from being thread-safe and
   there will be error when accessed from a differen thread:

thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `ThreadId(2)`,
 right: `ThreadId(1)`: rust_python_example::MyProgram is unsendable, but sent to another thread!', /Users/inikolaev/.cargo/registry/src/index.crates.io-6f17d22bba15001f/pyo3-0.19.2/src/impl_/pyclass.rs:927:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Exception in thread Thread-1 (worker):
Traceback (most recent call last):
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/threading.py", line 1038, in _bootstrap_inner
>>>     self.run()
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/threading.py", line 975, in run
    self._target(*self._args, **self._kwargs)
  File "<stdin>", line 2, in worker
pyo3_runtime.PanicException: assertion failed: `(left == right)`
  left: `ThreadId(2)`,
 right: `ThreadId(1)`: rust_python_example::MyProgram is unsendable, but sent to another thread!    
*/
#[pyclass]
struct MyProgram {
    program: Program
}

#[pymethods]
impl MyProgram {
    #[new]
    fn new(expr: String) -> Self {
        let program = Program::compile(&expr).unwrap();
        MyProgram { program }
    }

    fn evaluate(&mut self, ctx: &PyDict) -> PyResult<bool> {
        let mut context = Context::default();

        for (key, value) in ctx {
            if let Ok(name) = key.extract::<String>() {
                if let Ok(value) = value.extract::<bool>() {
                    context.add_variable(name, Value::Bool(value));
                } else if let Ok(value) = value.extract::<i32>() {
                    context.add_variable(name, Value::Int(value));
                } else if let Ok(value) = value.extract::<String>() {
                    context.add_variable(name, Value::String(value.into()));
                }
            }
        }

        let result = self.program.execute(&context);

        match result {
            Ok(Value::Bool(value)) => Ok(value),
            _ => Ok(false)
        }
    }
}

// Implement Send to tell compiler that it's thread-safe?
unsafe impl Send for MyProgram {}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_python_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyProgram>()?;
    Ok(())
}