use std::collections::HashMap;

// Define a trait for comparison functions
pub trait ComparisonFn {
    fn compare(&self, a: f64, b: f64) -> bool;
}

// Implement the trait for the specific comparison operations
pub struct LessThan;
impl ComparisonFn for LessThan {
    fn compare(&self, a: f64, b: f64) -> bool {
        a < b
    }
}

pub struct GreaterThan;
impl ComparisonFn for GreaterThan {
    fn compare(&self, a: f64, b: f64) -> bool {
        a > b
    }
}

pub struct LessThanOrEqual;
impl ComparisonFn for LessThanOrEqual {
    fn compare(&self, a: f64, b: f64) -> bool {
        a <= b
    }
}

pub struct GreaterThanOrEqual;
impl ComparisonFn for GreaterThanOrEqual {
    fn compare(&self, a: f64, b: f64) -> bool {
        a >= b
    }
}

pub struct Equal;
impl ComparisonFn for Equal {
    fn compare(&self, a: f64, b: f64) -> bool {
        a == b
    }
}

pub struct NotEqual;
impl ComparisonFn for NotEqual {
    fn compare(&self, a: f64, b: f64) -> bool {
        a != b
    }
}

// Struct to hold comparison functions
pub struct ComparisonFunctions {
    pub functions: HashMap<String, Box<dyn ComparisonFn>>,
}

impl ComparisonFunctions {
    pub fn new() -> Self {
        let mut functions: HashMap<String, Box<dyn ComparisonFn>> = HashMap::new();
        functions.insert("<".to_string(), Box::new(LessThan));
        functions.insert(">".to_string(), Box::new(GreaterThan));
        functions.insert("<=".to_string(), Box::new(LessThanOrEqual));
        functions.insert(">=".to_string(), Box::new(GreaterThanOrEqual));
        functions.insert("==".to_string(), Box::new(Equal));
        functions.insert("!=".to_string(), Box::new(NotEqual));
        ComparisonFunctions { functions }
    }

    // Method to get a comparison function by symbol
    pub fn get(&self, symbol: &str) -> Option<&Box<dyn ComparisonFn>> {
        self.functions.get(symbol)
    }
}



// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_less_than() {
        let lt = LessThan;
        assert_eq!(lt.compare(1.0, 2.0), true);
        assert_eq!(lt.compare(2.0, 1.0), false);
        assert_eq!(lt.compare(1.0, 1.0), false);
    }

    #[test]
    fn test_greater_than() {
        let gt = GreaterThan;
        assert_eq!(gt.compare(1.0, 2.0), false);
        assert_eq!(gt.compare(2.0, 1.0), true);
        assert_eq!(gt.compare(1.0, 1.0), false);
    }

    #[test]
    fn test_less_than_or_equal() {
        let lte = LessThanOrEqual;
        assert_eq!(lte.compare(1.0, 2.0), true);
        assert_eq!(lte.compare(2.0, 1.0), false);
        assert_eq!(lte.compare(1.0, 1.0), true);
    }

    #[test]
    fn test_greater_than_or_equal() {
        let gte = GreaterThanOrEqual;
        assert_eq!(gte.compare(1.0, 2.0), false);
        assert_eq!(gte.compare(2.0, 1.0), true);
        assert_eq!(gte.compare(1.0, 1.0), true);
    }

    #[test]
    fn test_equal() {
        let eq = Equal;
        assert_eq!(eq.compare(1.0, 2.0), false);
        assert_eq!(eq.compare(2.0, 1.0), false);
        assert_eq!(eq.compare(1.0, 1.0), true);
    }

    #[test]
    fn test_not_equal() {
        let neq = NotEqual;
        assert_eq!(neq.compare(1.0, 2.0), true);
        assert_eq!(neq.compare(2.0, 1.0), true);
        assert_eq!(neq.compare(1.0, 1.0), false);
    }

    #[test] // ComparisonFunctions
    fn test_comparison_functions_get() {
        let functions = ComparisonFunctions::new();
        assert_eq!(functions.get("<").unwrap().compare(1.0, 2.0), true);
        assert_eq!(functions.get(">").unwrap().compare(1.0, 2.0), false);
        assert_eq!(functions.get("<=").unwrap().compare(1.0, 2.0), true);
        assert_eq!(functions.get(">=").unwrap().compare(1.0, 2.0), false);
        assert_eq!(functions.get("==").unwrap().compare(1.0, 2.0), false);
        assert_eq!(functions.get("!=").unwrap().compare(1.0, 2.0), true);
    }

    
}


