enum IpTypeEnum {
    IPV4,
    IPV6,
}
enum IpTypeStringEnum {
    IPV4(String),
    IPV6(String),
}
enum IpTypeMultiEnum {
    IPV4(u8, u8, u8, u8), // Not a tuple, a distinct set of types.
    IPV6(String),
}
enum Message {
    Quit,                    // No data associated.
    Move { x: i32, y: i32 }, // Inline struct.
}
// The alternative.
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
fn route(ipKind: IpTypeEnum) {}
fn main() {
    let home_4: IpTypeEnum = IpTypeEnum::IPV4;
    let home_6: IpTypeEnum = IpTypeEnum::IPV6;
    route(home_4);
    route(home_6);
    let localhost: IpTypeStringEnum = IpTypeStringEnum::IPV4(String::from("127.0.0.1"));
    let loopback: IpTypeStringEnum = IpTypeStringEnum::IPV6(String::from("::1"));
    let no_number: Option<i32> = None;
    // Lots going on here. The expected type is an i32, but is currently empty.
    // The benefit is the compiler can infer what type no_number should be in the future.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // Error! Because y may be None in the future.
    // The solution? Match!
    let coin_one: Coin = Coin::Quarter(Option::Some(State::California));
    let coin_two: Coin = Coin::Quarter(Option::None);
    println!("Coin one is {}", coin_one.check_coin());
    println!("Coin two is {}", coin_two.check_coin());
    let config_max: Option<i32> = Option::Some(8_i32);
    if let Some(max) = config_max {
        println!("Config max is not None!");
    }
    // Sometimes this is unwise because it is not exhaustive.
    let Some(max) = config_max else {
        return (); // None instead of () also works.
    };
    // Does the same as above, but instead must return from the function if not matched.
}
impl Coin {
    fn check_coin(self: &Self) -> String {
        match self {
            Coin::Penny => String::from("Not a quarter!"),
            Coin::Nickel => String::from("Not a quarter!"),
            Coin::Dime => String::from("Not a quarter!"),
            Coin::Quarter(Some(State::California)) => String::from("A quarter with a state!"),
            Coin::Quarter(Some(State::Michigan)) => String::from("A quarter with a state!"),
            Coin::Quarter(Option::None) => String::from("A quarter without a state!"),
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
        // There's a rust analyzer bug which thinks None => None is wrong, but its not.
        // Not including a Some and a None for an Option<T> match will throw a compile time error.
    }
}

fn roll_die_catch_all_with_variable(value: i32) {
    match value {
        1 => println!("Rolled a 1!"),
        7 => println!("Rolled a 7!"),
        other => println!("Rolled a {}", other), // Variable is named for further use. Must be last
                                                 // and will warn if not used.
    }
}
fn roll_die_catch_all_without_variable(value: i32) {
    match value {
        1 => println!("Rolled a 1!"),
        7 => println!("Rolled a 7!"),
        _ => println!("Rolled a different number!"), // Variable is tossed. Must be last and will
                                                     // not warn if not used. A common convention
                                                     // to satisfy exhaustion is _ => ()
    }
}
enum State {
    California,
    Michigan,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<State>),
}
// Enums also use impl for associated functions.
// Option<T> is an included enum which has the type None or Some(T).
fn sort() -> String {
    let ip = IpTypeEnum::IPV4;
    match ip {
        IpTypeEnum::IPV4 => String::from("Yes!"),
        IpTypeEnum::IPV6 => String::from("No!"),
    }
}
