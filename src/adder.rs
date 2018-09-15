pub fn adder(numbers: &Vec<u64>) -> u128 {
    let mut number_holder: u128 = 0;

    for &item in numbers.iter() {
        number_holder += item as u128;
    }
    number_holder
}
