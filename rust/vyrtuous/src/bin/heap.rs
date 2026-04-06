fn main() {
    let s1: String = String::from("Hello");
    // The heap is expensive. The stack is cheap. Heap pointers are moved (variable size).
    // Stack variables are copied (fixed size)
    // Only one owner of a variable at a time (two variables can not have the same pointer).
    // The compiler will remove the original owner automatically when assigning a variable to the pointer.
    // Stack variables do not have ownership and therefore can have more than one owner
    let heap_primitive: Box<i32> = Box::new(8);
    // Use wrapping_* to provide the offset from the maximum or minimum.
    // Use checked_* to provide None if there is an overflow.
    // Use overflowing_* to return the value and boolean if there is an overflow.
    // Use saturation_* to hit the maximum or minimum of the type.
    let mut s: String = String::from("Hello");
    println!("{}", type_of(&s));
    s.push_str(", world!"); // Is likely annotated with -> usize 
    // Problem of systems programming languages:
    // Forgetting to free memory, or freeing it too early.
    // Doing these operations twice also is a bug.
    // One `allocate` needs to be paired with one `free`.
    // Scope dictates when memory should be freed. Resoucer Acquisition is Initialization (RAII).
    // Heap data reassignment is a `move` like a `shallow copy` with invalidation.
    // Stack data reassignment is a `copy` sometimes referred to as a `deep copy`.
    // Mutation to new data drops and reallocates memory.
    let s2: String = s.clone(); //Forced deep copy of a heap value. Expensive!
    // Stack variables have a `trait` called `Copy` which shows that its of fixed size. Copy is incompatible to Drop.
    let str: &String = &s2;
    // Instead of serving ownership to return_str, it passes the reference which maintains ownership.
    // This is borrowing. Borrowing a variable by reference must be immutable since no ownership exists.
    // See borrowing.rs.
}
