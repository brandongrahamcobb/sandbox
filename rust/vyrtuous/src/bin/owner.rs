fn main() {
    let s: String = String::from("Hello");
    print_string(s);
    println!("{} This should fail!", s);
}
fn print_string(s: String) {
    println!("{}", s);
}
