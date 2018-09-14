use std::io;

pub fn input() -> u64 {
    loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line.");

            match user_input.trim().parse() {
                Ok(num) => return num,
                Err(_) => {
                    println!("You must enter a positive integer.");
                    continue;
                }
            };
        }
}
