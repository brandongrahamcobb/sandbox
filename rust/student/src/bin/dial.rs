use std::fs::read_to_string;

fn main() {
    let mut code: u32 = 0;
    let mut index: u8 = 50;
    println!("{}", index);
    for line in read_to_string("input.txt").unwrap().lines() {
        let direction: &str = &line.to_string()[..1];
        let amount: i32 = line.to_string()[1..].trim().parse().unwrap();
        let right: bool;
        if direction == "R" {
            right = true;
        } else {
            right = false;
        }
        index = rotate(index, right, amount);
        println!(
            "Index = {}. Direction = {}, Amount = {}",
            &index, &direction, &amount
        );
        if index == 0 {
            code += 1;
        }
    }
    println!("{}", code);
}
fn rotate(index: u8, right: bool, amount: i32) -> u8 {
    let new_index: u8;
    if right {
        new_index = rotr(index, amount);
    } else {
        new_index = rotl(index, amount);
    }
    new_index
}

fn rotl(index: u8, amount: i32) -> u8 {
    let mut new_index: i32 = (index as i32 - amount) as i32;
    if new_index < 0 {
        let remainder = new_index % 100;
        if remainder != 0 {
            new_index = 100 + (new_index % 100);
        } else {
            new_index = 0;
        }
    }
    new_index as u8
}

fn rotr(index: u8, amount: i32) -> u8 {
    let mut new_index: i32 = (index as i32 + amount) as i32;
    if new_index > 99 {
        new_index = new_index % 100;
    }
    new_index as u8
}
