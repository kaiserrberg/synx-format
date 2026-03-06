//! SYNX Python binding — exposes parse/parse_active to Python via PyO3.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use synx_core::{self, Value, Mode, Options};

/// Convert synx_core::Value to a Python object.
fn value_to_py(py: Python<'_>, val: &Value) -> PyObject {
    match val {
        Value::Null => py.None(),
        Value::Bool(b) => b.into_pyobject(py).unwrap().to_owned().into_any().unbind(),
        Value::Int(n) => n.into_pyobject(py).unwrap().into_any().unbind(),
        Value::Float(f) => f.into_pyobject(py).unwrap().into_any().unbind(),
        Value::String(s) => s.into_pyobject(py).unwrap().into_any().unbind(),
        Value::Secret(_) => "[SECRET]".into_pyobject(py).unwrap().into_any().unbind(),
        Value::Array(arr) => {
            let items: Vec<PyObject> = arr.iter().map(|v| value_to_py(py, v)).collect();
            let list = PyList::new(py, &items).unwrap();
            list.into_pyobject(py).unwrap().into_any().unbind()
        }
        Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, value_to_py(py, v)).unwrap();
            }
            dict.into_pyobject(py).unwrap().into_any().unbind()
        }
    }
}

/// Parse a SYNX string and return a Python dict.
#[pyfunction]
fn parse(py: Python<'_>, text: &str) -> PyResult<PyObject> {
    let result = synx_core::parse(text);
    Ok(value_to_py(py, &result.root))
}

/// Parse a SYNX string as JSON. Returns a JSON string (faster for large files).
#[pyfunction]
fn parse_to_json(text: &str) -> PyResult<String> {
    let result = synx_core::parse(text);
    Ok(synx_core::to_json(&result.root))
}

/// Parse with engine resolution (!active mode).
#[pyfunction]
fn parse_active(py: Python<'_>, text: &str) -> PyResult<PyObject> {
    let mut result = synx_core::parse(text);
    if result.mode == Mode::Active {
        synx_core::resolve(&mut result, &Options::default());
    }
    Ok(value_to_py(py, &result.root))
}

/// SYNX Python module.
#[pymodule]
fn synx_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_active, m)?)?;
    Ok(())
}
