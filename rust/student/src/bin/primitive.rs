fn main() {
    // i8, i16, i32, i64, i128
    // u8, u16, u32, u64, u128
    let i8Max: i8 = i8::MAX;
    let i16Max: i16 = i16::MAX;
    let i32Max: i32 = i32::MAX;
    let i64Max: i64 = i64::MAX;
    println!(
        "i8: {}\ni16: {}\ni32: {}\ni64: {}",
        i8Max, i16Max, i32Max, i64Max
    );
    let u8Max: u8 = u8::MAX;
    let u16Max: u16 = u16::MAX;
    let u32Max: u32 = u32::MAX;
    let u64Max: u64 = u64::MAX;
    println!(
        "u8: {}\nu16: {}\nu32: {}\nu64: {}",
        u8Max, u16Max, u32Max, u64Max
    );
    let a: u8 = 0xff;
    let b: u8 = 0o77;
    let c: u8 = 0b1111_1111;
    // The number after the signiture of a type annotation determines the number of bits.
    // There are signed (i) integers and unsigned (u) integers. Integers can be represented by systems: hexidecimal, octal, binary, or decimal.
    // Types can also be paired with values following an underscore.
    // Compiler infers types from a lead type or if no lead type defined it defaults to default types.
    let x: u8 = u8::MAX;
    let y: u8 = 251u8;
    let z: i16 = 1_024_i16;
    let x: f32 = 0.2;
    let y: f32 = 0.1;
    let z1: f32 = 0.2 + 0.1;
    let x: f64 = 0.2;
    let y: f64 = 0.1;
    let z2: f64 = 0.2 + 0.1;
    // assert_eq!(z1, z2); // Fails because 0.1 as a fraction represented by binary is too precise and rounds the last place to the nearest binary size
    // Integer division rounds to the last integer which fits.
    // Statement comprehension works with boolean expressions, but you can't mix types.
    // See data_types.rs
}
