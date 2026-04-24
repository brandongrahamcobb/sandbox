// Rule 1: Only one mutable reference must exist per variable vs. one or more immutable references to the variable.
// Rule 2: All uses of a references after dropping are invalid.

pub mod test_module;

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
    // A package is a full implementation.
    // A crate is a tree of modules to make a library or executable.
    // It is the smallest amount of code the compiler considers at a time.
    // Normally considered as without any main().
    // main.rs is the starting point of a binary crate.
    // lib.rs is the library crate and there is only one at a time.
    // modules are looked at insize src/<module>.rs src/<module>/mod.rs
    // Or inline following mod <module> <implementation>.
    // Submodules are in src/<module>/<submodule>.rs or src/<module>/<submodule>/mod.rs.
    // A library is created via cargo new <module> --lib.
    // super referrs to the parent module.
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
