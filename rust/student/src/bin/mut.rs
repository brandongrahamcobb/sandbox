fn main() {
    let mut x: i32 = 5;
    let y: &mut i32 = &mut x;
    *y = 4;
    assert_eq!(x, 4);
}
