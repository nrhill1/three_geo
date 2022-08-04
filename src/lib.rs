
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
    base_length: f64,
    #[pyo3(get)]
    height: f64,
    // Apex & base of the pyramid
    #[pyo3(get)]
    apex: Point,
    #[pyo3(get)]
    base: arr[Point;4] 
}

#[pymethods]
impl Pyramid {
    #[new]
    pub fn new(base_length: f64, height: f64) -> Self {
        
        fn create_apex(&hgt: &f64) -> Point {
            Point{0, 0, &hgt}
        }

        let apex = create_apex(&height);

        fn create_base(&bl: &f64) {
            
            let sqrt = f64::sqrt;
            let bh = sqrt(&bl);

            let p1: Point = Point{x: &bh * -1, y: &bh * -1, z: 0};
            let p2: Point = Point{x: p1.x, y: p1.y - &bl, z: 0};
            let p3: Point = Point{x: p2.x + &bl, y: p2.y, z: 0};
            let p4: Point = Point{x: p3.x, y: p3.y + &bl, z: 0};

            let base: arr[Point;4] = [p1, p2, p3, p4]

            return base;
        }

        let base = create_base(&base_length)

        return Pyramid{base_length, height, apex, base}
    }
}


#[pymodule]
fn three_geo(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Pyramid>()?;

    Ok(())
}