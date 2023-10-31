// mod types;
// mod problem;

mod problem;

// import hashmap
use std::collections::HashMap;
fn main() {
    // Define the type for the comparison functions.
    // For example, this type specifies functions that take two i32 arguments and return a bool.
    type CompareFn = fn(i32, i32) -> bool;

    // Create a new HashMap mapping strings to the comparison functions.
    let mut operator_map: HashMap<&'static str, CompareFn> = HashMap::new();
    
    // Insert the operators into the map.
    // You have to wrap the actual comparison operators in functions that match the signature.
    operator_map.insert("==", |a, b| a == b);
    operator_map.insert("!=", |a, b| a != b);
    operator_map.insert(">", |a, b| a > b);
    operator_map.insert("<", |a, b| a < b);
    operator_map.insert("<=", |a, b| a <= b);
    operator_map.insert(">=", |a, b| a >= b);

    // Example of using the map
    if let Some(compare) = operator_map.get("==") {
        println!("Are 1 and 1 equal? {}", compare(1, 1));
    }
}