pub mod ceasar {
    use std::io;

    #[allow(dead_code)]
    pub fn run() {
        let mut input_str = String::new();

        println!("Please enter your secret text");
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");
        let clean_input = input_str.trim();

        //print!("\x1B[2J\x1B[1;1H"); //Clear secret from console

        let mut number_input = String::new();

        println!("Please enter your key");
        io::stdin()
            .read_line(&mut number_input)
            .expect("Failed to read line");

        let key: u32 = match number_input.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Invalid number!"),
        };

        let encrypted = encode(encrypt(decode(clean_input), key));
        println!("Encrypted: {}", encrypted);

        let decrypted = encode(decrypt(decode(&encrypted), key));
        println!("Decrypted: {}", decrypted);
    }

    fn decrypt(chars: Vec<char>, key: u32) -> Vec<char> {
        let mut encrypted: Vec<char> = Vec::new();
        for c in chars {
            let c_int = c as u32;
            let c_int_w_offset: u32;
            if key + c_int > char::MAX as u32 {
                c_int_w_offset = (c_int + key) - (char::MAX as u32)
            } else {
                c_int_w_offset = c_int + key;
            }
            encrypted.push(char::from_u32(c_int_w_offset).expect("Invalid Characters"))
        }

        encrypted
    }

    fn encrypt(chars: Vec<char>, key: u32) -> Vec<char> {
        let mut encrypted: Vec<char> = Vec::new();
        for c in chars {
            let c_int = c as u32;
            let c_int_w_offset: u32;
            if key > c_int {
                c_int_w_offset = ((char::MAX as u32) - key) + c_int
            } else {
                c_int_w_offset = c_int - key;
            }
            encrypted.push(char::from_u32(c_int_w_offset).expect("Invalid Characters"))
        }

        encrypted
    }

    fn decode(str: &str) -> Vec<char> {
        str.chars().collect::<Vec<char>>()
    }

    fn encode(chars: Vec<char>) -> String {
        let mut str = String::new();
        for c in chars {
            str.push(c)
        }
        str
    }
}
