use std::collections::HashMap;
use std::sync::Arc;
use pyo3::{prelude::*};
use cel_interpreter::{Context, Program, Value};
use cel_interpreter::objects::{Key, Map};

#[pyclass]
struct CelProgram {
    program: Arc<Program>
}

#[derive(FromPyObject)]
enum CelValue {
    CelBool {
        #[pyo3(attribute("value"))]
        value: bool,
    },
    CelInt {
        #[pyo3(attribute("value"))]
        value: i64,
    },
    CelString {
        #[pyo3(attribute("value"))]
        value: String,
    },
    CelFloat{
        #[pyo3(attribute("value"))]
        value: f64,
    },
    CelList{
        #[pyo3(attribute("value"))]
        value: Vec<CelValue>,
    },
    CelMap{
        #[pyo3(attribute("value"))]
        value: HashMap<String, CelValue>,
    },
}

pub trait ToCelValue {
    fn to_cel_value(&self) -> Value;
}

impl ToCelValue for CelValue {
    fn to_cel_value(&self) -> Value {
        match self {
            CelValue::CelBool {value} => {
                Value::Bool(*value)
            },
            CelValue::CelInt {value} => {
                Value::Int(*value)
            },
            CelValue::CelFloat {value} => {
                Value::Float(*value)
            },
            CelValue::CelString {value} => {
                Value::String((*value).clone().into())
            },
            CelValue::CelList {value} => {
                Value::List(value.iter().map(|x| x.to_cel_value()).collect::<Vec<Value>>().into())
            },
            CelValue::CelMap {value} => {
                let mut map = HashMap::new();
                map.extend(value.iter().map(|(k, v)| (Key::String((*k).clone().into()), v.to_cel_value())));
                Value::Map(Map::from(map))
            },
        }
    }
}

#[pymethods]
impl CelProgram {
    #[new]
    fn new(expr: String) -> Self {
        let program = Arc::new(Program::compile(&expr).unwrap());
        CelProgram { program }
    }

    fn evaluate(&mut self, ctx: HashMap<String, CelValue>) -> PyResult<bool> {
        let mut context = Context::default();

        for (name, value) in ctx {
            context.add_variable(name, value.to_cel_value());
        }

        let result = self.program.execute(&context);

        match result {
            Ok(Value::Bool(value)) => Ok(value),
            _ => Ok(false)
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pycel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CelProgram>()?;
    Ok(())
}