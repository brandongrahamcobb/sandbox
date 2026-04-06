fn main() {
    // A function and a method are different. A method is an associated function specific to a struct.
    // Structs are like named tuples defined via
    struct Person {
        name: String,
        age: i32,
    }
    let person_one = Person {
        // In structs, I think I prefer the _type. I don't really like how Strings can't be annotated this way in structs.
        name: String::from("Brandon"),
        age: 27_i32,
    };
    // The entire struct must be mutable for changes to the individual fields.
    // Shorthand allows for field instead of field: field if the field names are the same.
    let person_two = Person {
        name: String::from("Allison"), // Updates the value to a new value.
        ..person_one // !!Moves!! the heap values and !!Copies!! stack values from the previous person_one instance.
    };
    // Tuple structs are structs with only type annotations no names.
    #[derive(Debug)]
    struct Color(i32, i32, i32); // Color in this case is the type of the struct which cannot be passed as a variable of a non-Color type.
    let origin = Color(0, 0, 0); // Requires naming for destructuring
    let Color(x, y, z) = origin;
    struct TraitSpecific; // Unit-like structs. Like () for tuples.
    println!("Color is {origin:?}."); // :? is used for Debug formatting but it requires: #[derive(Debug)]
}
