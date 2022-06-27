#[allow(dead_code)]
pub mod list {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::io;

    
    type NumUwU = i32;

    pub fn run() {
        let mut int_list: Vec<NumUwU> = Vec::new();

        let mut number_input: String;

        println!("Please enter all numbers and press enter after each number, when you're finished just press enter a second time.");

        loop {
            number_input = String::new();
            print!("\x1B[2J\x1B[1;1H");
            print!("{:#?}", int_list);

            io::stdin()
                .read_line(&mut number_input)
                .expect("Failed to read line");

            let entered_number: NumUwU = match number_input.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            int_list.push(entered_number);
        }

        print!("\x1B[2J\x1B[1;1H");
        println!(
            "Successfully parsed {} numbers, now sorting",
            int_list.len()
        );

        // gnome sort bcs why not
        // int_list.sort(); is cringe
        let mut i: usize = 1;
        while i < int_list.len() {
            let before = int_list[i - 1];
            let curr = int_list[i];

            match before.cmp(&curr) {
                Ordering::Greater => {
                    int_list[i - 1] = curr;
                    int_list[i] = before;
                    if i > 1 {
                        i -= 1
                    }
                }
                Ordering::Less => i += 1,
                Ordering::Equal => i += 1,
            }
        }

        println!("{:?}", int_list);

        let median = int_list[int_list.len() / 2];
        println!("Median: {}", median);

        let mut occurences_map: HashMap<NumUwU, usize> = HashMap::new();
        for x in int_list {
            let ent = occurences_map.entry(x).or_insert(0);
            *ent += 1;
        }

        let mut mode = 0;
        {
            let mut highest_count = 0;
            for (x, y) in &occurences_map {
                if y > &highest_count {
                    mode = *x;
                    highest_count = *y;
                }
            }
        }

        println!("Mode:   {}", mode);
    }
}
