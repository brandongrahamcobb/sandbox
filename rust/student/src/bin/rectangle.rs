#[derive(Debug)] // Required for struct debug printing. Does not require any `use` keyword.
struct Rectangle {
    width: i32,
    height: i32,
}
struct RectangleTuple(i32, i32);
fn main() {
    calculate_area_from_struct();
    calculate_area_from_tuple();
    calculate_area_from_struct_tuple();
}
fn calculate_area_from_struct() {
    let rectangle = Rectangle {
        width: 10_i32,
        height: 5_i32,
    };
    let area: i32 = area_struct(&rectangle);
    println!("{}", area);
    println!("{rectangle:?}");
    let area: i32 = rectangle.area();
    let new_rectangle = Rectangle {
        width: 5_i32,
        height: 4_i32,
    };
    if rectangle.can_hold(&new_rectangle) {
        println!("Rectangle {rectangle:?} is big enough for {new_rectangle:?}!");
    } else {
        println!("Rectangle {rectangle:?} is too small for {new_rectangle:?}!");
    }
    // Automatic referencing and dereferencing allows for (*rectangle).area() or rectangle.area().
    // The compiler infers the referencing based on whether or not rectangle is a reference or the owner of the variable.
    // Reading (&self), mutating (&mut self) or consuming (self).
    let dimension: i32 = 5;
    let sqr: Rectangle = Rectangle::square(dimension);
}
fn calculate_area_from_tuple() {
    let dimensions: (i32, i32) = (5, 10);
    let area: i32 = area_tuple(&dimensions);
    println!("{}", area);
}
fn calculate_area_from_struct_tuple() {
    let rectangle: RectangleTuple = RectangleTuple(5, 10);
    let area: i32 = area_struct_tuple(&rectangle);
    println!("{}", area);
}
fn area_struct_tuple(rectangle: &RectangleTuple) -> i32 {
    let area: i32 = rectangle.0 * rectangle.1;
    area
}
fn area_tuple(dimensions: &(i32, i32)) -> i32 {
    let area: i32 = dimensions.0 * dimensions.1;
    area
}
fn area_struct(rectangle: &Rectangle) -> i32 {
    let area: i32 = rectangle.width * rectangle.height;
    area
}
impl Rectangle {
    // Called `associated functions`.
    fn area(self: &Self) -> i32 {
        // &Self is technically &Rectangle. Both work, but idiomatic Rust uses &Self.
        let area: i32 = self.width * self.height;
        area
    }
    fn can_hold(self: &Self, rectangle: &Rectangle) -> bool {
        let original_area = self.area();
        let new_area = rectangle.area();
        self.width >= rectangle.width && self.height >= rectangle.width && original_area >= new_area
    }
    // Optionally, can not take &self and therefore are not `methods`.
    // An example would be ::from for String::from.
    // Often called `new` (idiomatically) and are likely constructors.
    fn square(dimension: i32) -> Self {
        Self {
            width: dimension,
            height: dimension,
        }
    }
}
