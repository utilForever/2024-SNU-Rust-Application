// Calculate the magnitude of the given vector.
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;

    for coord in vector {
        mag_squared += coord * coord;
    }

    mag_squared.sqrt()
}

// Change the magnitude of the vector to 1.0 without changing its direction.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);

    for item in vector {
        *item /= mag;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_normal() {
        let mut vec = [1.0, 2.0, 9.0];
        assert!((magnitude(&mut vec) - 9.273618495495704).abs() < f64::EPSILON);
    }

    #[test]
    fn normalize_normal() {
        let mut vec = [1.0, 2.0, 9.0];
        normalize(&mut vec);
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn magnitude_unit() {
        let mut vec = [0.0, 1.0, 0.0];
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn normalize_unit() {
        let mut vec = [0.0, 1.0, 0.0];
        normalize(&mut vec);
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }
}
