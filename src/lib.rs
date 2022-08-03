
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn format_coords(a: usize, b: usize) -> PyResult<String> {
    Ok(format!("{} {}", a, b))
}

/// A Python module implemented in Rust.
#[pymodule]
fn three_geo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format_coords, m)?)?;
    Ok(())
}