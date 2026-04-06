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
