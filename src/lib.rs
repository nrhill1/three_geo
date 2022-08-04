
use pyo3::prelude::*;



#[pymodule]
fn three_geo(py: Python, m: &PyModule) -> PyResult<()> {

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

    // #[pyfn(m, "format_coords")]
    // fn format_coords(_py: Python, x:f64, y:f64, z:f64) -> PyResult<String> {
        
    // }

    Ok(())
}