use pyo3::prelude::*;

// Point definition
#[pyclass]
#[derive(Copy, Clone)]
struct Point {
    #[pyo3(get)]
    x: f64,
    #[pyo3(get)]
    y: f64,
    #[pyo3(get)]
    z: f64,
}


// Point implementation
#[pymethods]
impl Point {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x: x, y: y, z: z }
    }

    #[staticmethod]
    pub fn coords(self: &Point) -> [f64;3] {
        let coords: [f64;3] = [self.x, self.y, self.z];
        return coords;
    }

}

// Pyramid definition
#[pyclass]
struct Pyramid {
    // Parameters for new pyramid
    #[pyo3(get)]
    base_length: f64,
    #[pyo3(get)]
    height: f64,
    // Apex  base of the pyramid
    #[pyo3(get)]
    apex: Point,
    #[pyo3(get)]
    base: Vec<Point>
}

// Pyramid implementation
#[pymethods]
impl Pyramid {
    #[new]
    pub fn new(base_length: f64, height: f64) -> Self {
        
        fn create_apex(hgt: f64) -> Point {
            Point{x: 0.0, y: 0.0, z: hgt}
        }

        let apex: Point = create_apex(height);

        fn create_base(bl: f64) -> Vec<Point> {
            
            let sqrt = f64::sqrt;
            let bh = sqrt(bl);

            let p1: Point = Point{x: bh * -1.0, y: bh * -1.0, z: 0.0};
            let p2: Point = Point{x: p1.x, y: p1.y - bl, z: 0.0};
            let p3: Point = Point{x: p2.x + bl, y: p2.y, z: 0.0};
            let p4: Point = Point{x: p3.x, y: p3.y + bl, z: 0.0};

            let base: Vec<Point> = vec![p1, p2, p3, p4];

            return base
        }

        let base: Vec<Point> = create_base(base_length);

        return Pyramid{base_length: base_length, height: height, apex: apex, base: base}
    }

    pub fn surface_area(&self) -> f64 {
        let sqrt = f64::sqrt;

        let area: f64 = f64::powf(self.base_length, 2.0) + 2.0 * self.base_length * sqrt(((f64::powf(self.base_length, 2.0))/4.0)+self.height);

        return area
    }
}


#[pymodule]
fn three_geo(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Pyramid>()?;

    Ok(())
}