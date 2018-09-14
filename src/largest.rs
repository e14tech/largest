pub fn largest(numbers: &Vec<u64>) -> u64 {
    let mut largest = numbers[0];

    for &item in numbers.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
