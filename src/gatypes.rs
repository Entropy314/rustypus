// import random crate
use rand::Rng;
// import the powf function from the f64 module


// Create Enum for types called SolutionType
#[derive(Debug)]
pub enum SolutionDataTypes {
    Real(Real),
    BitBinary(BitBinary),
    Integer(Integer)
}

impl SolutionDataTypes {
    pub fn new_real(value: Option<f64>, lower_bound: Option<f64>, upper_bound: Option<f64>) -> Self {
        SolutionDataTypes::Real(Real::new(value, lower_bound, upper_bound))
    }

    pub fn new_integer(value: Option<i64>, lower_bound: Option<i64>, upper_bound: Option<i64>) -> Self {
        SolutionDataTypes::Integer(Integer::new(value, lower_bound, upper_bound))
    }

    pub fn new_binary(value: Option<i64>) -> Self {
        SolutionDataTypes::BitBinary(BitBinary::new(value))
    }
   
}

pub enum RealDataType { 
    F64, 
    F32, 
    F128, 
    F256,
    F16

}

pub enum IntegerDataType { 
    I64, 
    I32, 
    I16, 
    I8, 
    U64, 
    U32, 
    U16, 
    U8

}

pub enum BinaryDataType { 
    Bool, 
    U8,
    U4, 
    U2, 
    U16, 
    U32, 
    U64, 
    U128,
    U256
}
// pub enum SolutionDataTypes { 
//     F64, 
//     I64,
//     Bool,
//     I32, 
//     U32,
//     F32

// }

#[derive(Debug)]
pub struct BitBinary { 
    pub value: Option<i64>
}

// create a method on BitBinary that randomly genrates a 1 or 0 for value
impl BitBinary {
    pub fn new(value: Option<i64>) -> Self {
        let mut rng = rand::thread_rng();

        let value = match value {
            Some(v) => {
                if v != 0 && v != 1 {
                    panic!("value must be 0 or 1");
                }
                Some(v)
            },
            None => Some(rng.gen_range(0..=1)),
        };

        BitBinary { value }
    }

    pub fn value(&self) -> Option<i64> {
        self.value
    }
}


// Create an Integer object where the lower and upper bounds are optional parameters
#[derive(Debug)]
pub struct Integer {
    pub value: Option<i64>,
    pub lower_bound: Option<i64>,
    pub upper_bound: Option<i64>
}

// create a method on Integer that randomly genrates a number between the lower and upper bounds
impl Integer {
    pub fn new(value: Option<i64>, lower_bound: Option<i64>, upper_bound: Option<i64>) -> Self {
        // if lower_bound == upper_bound panic
        if lower_bound.is_some() && upper_bound.is_some() && lower_bound >= upper_bound {
            panic!("lower_bound must be less than upper_bound");
        }

        let mut rng = rand::thread_rng();
        // if value is None generate value between lower and upper bounds else use value
        let value = match value {
            Some(v) => {
                if lower_bound.is_some() && v < lower_bound.unwrap() || upper_bound.is_some() && v > upper_bound.unwrap() {
                    panic!("value must be between the lower and upper bounds");
                }
                Some(v)
            },
            None => {
                match (lower_bound, upper_bound) {
                    (Some(lower_bound), Some(upper_bound)) => Some(rng.gen_range(lower_bound..upper_bound)),
                    (Some(lower_bound), None) => Some(rng.gen_range(lower_bound..i64::MAX)),
                    (None, Some(upper_bound)) => Some(rng.gen_range(i64::MIN..upper_bound)),
                    (None, None) => Some(rng.gen_range(i64::MIN..i64::MAX)),
                }
            }
        };

        Integer { value, lower_bound, upper_bound }
    }

    // create a method on Integer that returns the value
    pub fn value(&self) -> Option<i64> {
        self.value
    }
}

#[derive(Debug)]
pub struct Real { 
    pub value: Option<f64>, 
    pub lower_bound: Option<f64>,
    pub upper_bound: Option<f64>
}

// create a method on Real that randomly genrates a number between the lower and upper bounds
impl Real {
    pub fn new(value: Option<f64>, lower_bound: Option<f64>, upper_bound: Option<f64>) -> Self {
        // if lower_bound == upper_bound panic
        
        if lower_bound.is_some() && upper_bound.is_some() && lower_bound >= upper_bound {
            panic!("lower_bound must be less than upper_bound");
        }
        // if lower_bound or upper_bound is None set to min or max value to self struct
        

        if lower_bound.is_none() { 
            let lower_bound = Some(f64::MIN);
        }
        if upper_bound.is_none() { 
            let upper_bound = Some(f64::MAX);
        }

        let mut rng = rand::thread_rng();
        // if lower_bound is None and upper_bound is NOne max and min values are used



        // if value is None generate value between lower and upper bounds else use value
        let value = match value {
            Some(v) => {
                if lower_bound.is_some() && v < lower_bound.unwrap() || upper_bound.is_some() && v > upper_bound.unwrap() {
                    panic!("value must be between the lower and upper bounds");
                }
                Some(v)
            },
            None => {
                match (lower_bound, upper_bound) {
                    (Some(lower_bound), Some(upper_bound)) => Some(rng.gen_range(lower_bound..upper_bound)),
                    (Some(lower_bound), None) => Some(rng.gen_range(lower_bound..f64::MAX)),
                    (None, Some(upper_bound)) => Some(rng.gen_range(f64::MIN..upper_bound)),
                    (None, None) => Some(rng.gen_range(f64::MIN..f64::MAX)),
                }
            }
        };

        Real { value, lower_bound, upper_bound }
    }

    // create a method on Real that returns the value
    pub fn value(&self) -> Option<f64> {
        self.value
    }
}



// Create Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::gatypes::SolutionDataTypes;
    #[test]
    fn test_binary_no_default() {
        let BitBinary = BitBinary::new(None);
        println!("{:?}",BitBinary.value);
        assert!(BitBinary.value() == Some(0) || BitBinary.value() == Some(1));
    }

    #[test]
    fn test_binary_default() {
        let BitBinary = BitBinary::new(Some(1));
        assert!(BitBinary.value() == Some(0) || BitBinary.value() == Some(1));
    }

    #[test]
    fn test_integer_no_default() {
        let integer = Integer::new(None, Some(0), Some(10));
        assert!(integer.value >= Some(0) && integer.value <= Some(10));
    }

    #[test]
    fn test_integer_default() {
        let integer = Integer::new(Some(8888), Some(0), Some(9000));
        assert!(integer.value >= Some(0) && integer.value <= Some(9000));
    }

    #[test]
    fn test_real_no_default() { 
        let real = Real::new(None,Some(0.0), Some(2000.0));
        println!("{:?}",real.value);
        assert!(real.value >= Some(0.0) && real.value <= Some(2000.0));
    }

    #[test]
    fn test_real_default() { 
        let real = Real::new(Some(400.4),Some(0.0), Some(6000.0));
        println!("{:?}",real.value);
        assert!(real.value >= Some(0.0) && real.value <= Some(6000.0));
        assert!(real.value == Some(400.4));
    }
    // Unit test for the Enum
    #[test]
    fn test_solution_type() {
        let BitBinary:BitBinary = BitBinary { value: Some(1) };
        let integer:Integer = Integer { value: Some(10), lower_bound: Some(0), upper_bound: Some(100) };
        let real:Real = Real { value: Some(10.0), lower_bound: Some(0.0), upper_bound: Some(100.0) };

        // print the type
        println!("{:?}", BitBinary);
        println!("{:?}", integer);
        println!("{:?}", real);
        assert!(match BitBinary {
            _BitBinary => true,
            _ => false
        });
        assert!(match integer {
            _Integer => true,
            _ => false
        });
        assert!(match real {
            _Real => true,
            _ => false
        });
    }

    // test solutiontype enum

    // Now for the SolutionType enum
    #[test]
    fn test_solution_type2() {
        let real: SolutionDataTypes = SolutionDataTypes::new_real(None, Some(0.0), Some(10.0));
        let binary: SolutionDataTypes = SolutionDataTypes::new_binary(Some(1));
        let integer: SolutionDataTypes = SolutionDataTypes::new_integer(Some(10), Some(0), Some(100));
        let real: Real = Real::new(None, Some(0.0), Some(10.0));

        // print the type
        println!("{:?}", real);
        println!("{:?}", binary);
        println!("{:?}", integer);
        assert!(match real {
            _Real => true,
            _ => false
        });
        assert!(match binary {
            _BitBinary => true,
            _ => false
        });
        assert!(match integer {
            _Integer => true,
            _ => false
        });
    }

}
