// import random crate
use rand::Rng;

// Create Enum for types called SolutionType
#[derive(PartialEq, Eq)]
pub enum SolutionType {
    Binary,
    Integer,
    Real
}


pub struct Binary { 
    value: i64 
}

// create a method on Binary that randomly genrates a 1 or 0 for value
impl Binary {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let value: i64 = rng.gen_range(0..2);
        Binary { value }
    }
}

// create a method on Binary that returns the value
impl Binary {
    pub fn value(&self) -> i64 {
        self.value
    }
}

// Create an Integer object where the lower and upper bounds are optional parameters
pub struct Integer {
    value: i64,
    lower_bound: Option<i64>,
    upper_bound: Option<i64>
}

// create a method on Integer that randomly genrates a number between the lower and upper bounds
impl Integer {
    pub fn new(lower_bound: Option<i64>, upper_bound: Option<i64>) -> Self {
        let mut rng = rand::thread_rng();
        // if lower_bound == upper_bound panic
        if lower_bound >= upper_bound {
            panic!("lower_bound and upper_bound cannot be greater than or equal");
        }
        let value: i64 = match (lower_bound, upper_bound) {
            (Some(lower_bound), Some(upper_bound)) => rng.gen_range(lower_bound..upper_bound),
            (Some(lower_bound), None) => rng.gen_range(lower_bound..i64::MAX),
            (None, Some(upper_bound)) => rng.gen_range(i64::MIN..upper_bound),
            (None, None) => rng.gen_range(i64::MIN..i64::MAX)
        };
        // print value
        // println!("Random value: {}", value);
        Integer { value, lower_bound, upper_bound }
    }

    // create a method on Integer that returns the value
    pub fn value(&self) -> i64 {
        self.value
    }
}

pub struct Real { 
    value: f64, 
    lower_bound: Option<f64>,
    upper_bound: Option<f64>
}

// create a method on Real that randomly genrates a number between the lower and upper bounds
impl Real {
    pub fn new(lower_bound: Option<f64>, upper_bound: Option<f64>) -> Self {
        let mut rng = rand::thread_rng();
        // if lower_bound == upper_bound panic
        if lower_bound == upper_bound {
            panic!("lower_bound and upper_bound cannot be equal");
        }
        let value: f64 = match (lower_bound, upper_bound) {
            (Some(lower_bound), Some(upper_bound)) => rng.gen_range(lower_bound..upper_bound),
            (Some(lower_bound), None) => rng.gen_range(lower_bound..f64::MAX),
            (None, Some(upper_bound)) => rng.gen_range(f64::MIN..upper_bound),
            (None, None) => rng.gen_range(f64::MIN..f64::MAX)
        };

        Real { value, lower_bound, upper_bound }
    }

    // create a method on Real that returns the value
    pub fn value(&self) -> f64 {
        self.value
    }
}

// Create Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary() {
        let binary = Binary::new();
        assert!(binary.value() == 0 || binary.value() == 1);
    }

    #[test]
    fn test_integer() {
        let integer = Integer::new(Some(0), Some(10));
        assert!(integer.value >= 0 && integer.value <= 10);
    }

    #[test]
    fn test_real() { 
        let real = Real::new(Some(0.0), Some(2000.0));
        assert!(real.value >= 0.0 && real.value <= 2000.0);
    }
    // Unit test for the Enum
    #[test]
    fn test_solution_type() {
        let binary = SolutionType::Binary;
        let integer = SolutionType::Integer;
        let real = SolutionType::Real;
        assert!(match binary {
            SolutionType::Binary => true,
            _ => false
        });
        assert!(match integer {
            SolutionType::Integer => true,
            _ => false
        });
        assert!(match real {
            SolutionType::Real => true,
            _ => false
        });
    }

}
