mod guess_the_number {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    #[allow(dead_code)]
    pub fn run() {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..101); //Equivalent to 1..=100

        let mut guess: String;

        println!("Please input your guess.");

        loop {
            guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess_number: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess_number.cmp(&secret_number) {
                Ordering::Less => println!("Guess higher"),
                Ordering::Greater => println!("Guess lower"),
                Ordering::Equal => {
                    println!("Guess was correct!");
                    break;
                }
            }
        }
    }
}
