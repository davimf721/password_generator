use std::char::from_u32;
use  rand::Rng;
use  rand::{thread_rng};
use std::io;

fn main() {
    println!("Password Generator!");

    println!("Please put the length: ");

    let mut password_length =String::new();
    io::stdin()
        .read_line(&mut password_length)
        .expect("Failed to read line");

    let mut result = String::new();

    for _ in 0..password_length {
        let number = thread_rng().gen_range(48..122);
        let ch = from_u32(number).unwrap();
        result.push(ch);
    }
    println!("{}", result);
}