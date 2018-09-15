pub fn mean(numbers: &Vec<u64>, input: &u64) -> f64 {
    let mean: f64;
    let mut adder: u128 = 0;

    for &item in numbers.iter() {
        adder += item as u128;
    }
    mean = adder as f64 / *input as f64;
    mean
}
