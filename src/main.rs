extern crate rand;

mod input;
mod largest;

use input::*;
use largest::*;
use rand::Rng;

fn main() {
    let mut numbers: Vec<usize> = Vec::new();
    println!("How many numbers should I randomly generate?");
    let user_input = input();

    for _ in 0..user_input {
        numbers.push(rand::thread_rng().gen_range(0, 100000000000001));
    }
    let largest_number = largest(&numbers);

    println!("The largest number in the list you entered is: {}.", largest_number);
}
