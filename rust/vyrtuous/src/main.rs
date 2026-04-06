// Rule 1: Only one mutable reference must exist per variable vs. one or more immutable references to the variable.
// Rule 2: All uses of a references after dropping are invalid.

fn main() {
    println!("Hello, world!");
    // Snake case for function and variable names.
    // Parameters must be type annotated.
    // Expressions evaluate to a result. Statements do not return a value.
    // No colon for return values unless explicitly returning early with 'return'.
    // Pointers are stored on the stack.
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
