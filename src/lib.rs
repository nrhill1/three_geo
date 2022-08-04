
use pyo3::prelude::*;

// Point definition
#[pyclass]
struct Point {
    #[pyo3(get)]
    x: f64,
    #[pyo3(get)]
    y: f64,
    #[pyo3(get)]
    z: f64
}

// Point implementation
#[pymethods]
impl Point {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z}
    }
}

#[pyclass]
struct Pyramid {
    // Parameters for new pyramid
    #[pyo3(get)]
    apex: Point,
    #[pyo3(get)]
    base_length: f64,
    #[pyo3(get)]
    hypotenuse: f64,
    //
    #[pyo3(get)]
    p1: Point,
    #[pyo3(get)]
    p2: Point,
    #[pyo3(get)]
    p3: Point,
    #[pyo3(get)]
    p4: Point
}

#[pymethods]
impl Pyramid {
    #[new]
    pub fn new()
}


#[pymodule]
fn three_geo(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;

    Ok(())
}