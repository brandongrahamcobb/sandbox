fn main() {
    let mut s: String = String::from("Hello, world!");
    let s1: &String = &mut s;
    let s2: &String = &mut s; // Double !!mutable!! reference. Illegal if s1 AND s2 is used! The trick is {} scoping.
    // Multiple immutable references are allowed because the data never changes.
    // In general, functions don't take ownership of their arguments unless neccesary.
    fn first_word(s: &String) -> &str {
        // We return &str because ownership was not provided to the method. Otherwise it would be &String.
        let bytes = s.as_bytes(); // For slicing
        for (i, &item) in bytes.iter().enumerate() {
            // Iter doesn't need ownership of item so its a reference.
            if item == b' ' {
                return i;
            }
        }
        s.len() // The key is this is not &s.
    }
    s.clear();
    // If s is cleared, the returned value of first_word is no longer representative of the data obtained from first_word because borrowing ends upon returning a value not a reference. The solution is a string slice because first_word returns a reference.
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..] // The key is this is &s.
    }
    // This is a reference to rule 1.
    let s: String = String::from("The brown fox jumped over the lazy dog");
    let segment: String = slice(&s);
    println!("{}", segment);
    fn slice(s: &str) -> &str {
        let bytes = s.as_bytes();
        let mut segments: [&str; 8] = [""; 8];
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                segments[i] = &s[0..i]
            }
        }
        &segments[0]
    }
    let s = "Hello world"; // Type is &str because the string is a slice and the string (not the variable s) is an immutable reference to the string on the heap. s is simply forced to be one mutable reference or an immutable reference which are both valid because it satisfies rule 1. If s was mut, and it was possible to reference the string by doing "Hello world" again, it would fail if another var like let mut t = "Hello world"; due to rule 1. This is safely guarded because reusing the data creates a new variable not a reference.
    fn first_word(s: &str) -> &str { // Using &str for the parameter is idiomatic Rust because it allows for the passed in string to be a slice or as a reference to the owner, unlike &String which only allows for a reference to the owner.
    }
    // An &mut annotation allowes for a borrowed reference to be changed. The trick is the value can only have one reference.
    // This is the end of a chapter.
}
