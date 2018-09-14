extern crate rand;

mod input;
mod largest;
mod smallest;
mod adder;

use input::*;
use largest::*;
use smallest::*;
use adder::*;
use rand::Rng;

fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    println!("How many numbers should I randomly generate?");
    let user_input = input();

    for _ in 0..user_input {
        numbers.push(rand::thread_rng().gen_range(0, 100000000000001));
    }
    println!("Finding largest number.");

    let largest_number = largest(&numbers);

    println!("Finding smallest number.");

    let smallest_number = smallest(&numbers);

    println!("Adding all the numbers up.");

    let added_number = adder(&mut numbers);

    println!("The largest number in the list: {}.", largest_number);
    println!("The smallest number in the list is: {}", smallest_number);
    println!("All the numbers added up is: {}", added_number);
}
