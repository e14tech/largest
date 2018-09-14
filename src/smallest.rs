pub fn smallest(numbers: &Vec<u64>) -> u64 {
    let mut smallest = numbers[0];

    for &item in numbers.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}
