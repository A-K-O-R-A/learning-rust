pub mod pig_latin {
    use std::io;

    static VOWL_LIST: &str = "aeiou";

    #[allow(dead_code)]
    pub fn run() {
        println!("What do you want to translate to pig latin?");
        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        // split into words
        // if in vowel list "-hay" at the end
        // else take first char, put "-{c}ay" at the end
        // utf8?

        let mut output_str = String::new();
        for word in input_str.split_whitespace() {
            let first_char = word.chars().next();
            if let Some(c) = first_char {
                if VOWL_LIST.contains(c) {
                    output_str.push_str(word);
                    output_str.push_str("-hay");
                } else {
                    output_str.push_str(&word[1..(word.len())]);
                    output_str.push_str(format!("-{}ay", c).as_str());
                }
            }
            output_str.push_str(" ");
        }
        println!("{}", output_str);

        //print!("\x1B[2J\x1B[1;1H");
    }
}
