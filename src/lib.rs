
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn format_coords(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn o3_geo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format_coords, m)?)?;
    Ok(())
}