// pyo3 
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
#[derive(Clone)]
struct Polygon {
    #[pyo3(get)]
    points: Vec<Point>
}


#[pymethods]
impl Polygon {
    #[new]
    pub fn new(points: Vec<Point>) -> Self {
        return Polygon {points}
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
    base: Vec<Point>,
    #[pyo3(get)]
    x_off: f64,
    #[pyo3(get)]
    y_off: f64,
}

// Pyramid implementation
#[pymethods]
impl Pyramid {
    #[new]
    pub fn new(base_length: f64, height: f64, x_off: f64, y_off: f64) -> Self {
        
        fn create_apex(x_off: f64, y_off: f64, height: f64) -> Point {
            Point{x: x_off, y: y_off, z: height}
        }

        let apex: Point = create_apex(x_off, y_off, height);

        fn create_base(bl: f64, x_off: f64, y_off: f64) -> Vec<Point> {
            
            let sqrt = f64::sqrt;
            let bh = sqrt(bl);

            let p1: Point = Point{x: bh * -1.0 + x_off, y: bh * -1.0 + y_off, z: 0.0};
            let p2: Point = Point{x: p1.x, y: p1.y - bl, z: 0.0};
            let p3: Point = Point{x: p2.x + bl, y: p2.y, z: 0.0};
            let p4: Point = Point{x: p3.x, y: p3.y + bl, z: 0.0};

            let base: Vec<Point> = vec![p1, p2, p3, p4];

            return base
        }

        let base: Vec<Point> = create_base(base_length, x_off, y_off);

        return Pyramid {base_length, height, apex, base, x_off, y_off}
    }
    
    pub fn base_area(&self) -> f64 {
        let ba: f64 = f64::powf(self.base_length, 2.0);
        
        return ba
    }

    pub fn surface_area(&self) -> f64 {
        let sqrt = f64::sqrt;

        let surf_area: f64 = f64::powf(self.base_length, 2.0) 
            + 2.0 * self.base_length 
            * sqrt(((f64::powf(self.base_length, 2.0))/4.0)
            + self.height);

        return surf_area
    }

    pub fn volume(&self) -> f64 {
        let vol: f64 = (f64::powf(self.base_length, 2.0) * self.height)/(3.0);
        return vol
    }
}


#[pymodule]
fn three_geo(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Polygon>()?;
    m.add_class::<Pyramid>()?;
    

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let point = Point::new(1.0, 2.0, 3.4);

        assert_eq!(1.0, point.x);
        assert_eq!(2.0, point.y);
        assert_eq!(3.4, point.z);
    }

    #[test]
    fn test_pyramid() {
        let pyramid = Pyramid::new( 1.0, 2.5, 1.9, -2.5 );

        assert_eq!(1.9, pyramid.apex.x);
        assert_eq!(-2.5, pyramid.apex.y);
        assert_eq!(2.5, pyramid.apex.z);
        
    }
}

