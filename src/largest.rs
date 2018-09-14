pub fn largest(numbers: &Vec<usize>) -> usize {
    let mut largest = numbers[0];

    for &item in numbers.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
