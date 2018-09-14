use std::io;

pub fn input() -> u64 {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line.");

    match user_input.trim().parse() {
        Ok(num) => return num,
        Err(_) => panic!("Did not enter an integer!"),
    };
}
