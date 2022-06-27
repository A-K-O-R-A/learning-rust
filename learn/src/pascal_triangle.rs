pub mod pascal_triangle {
    use std::io;

    #[allow(dead_code)]
    pub fn run() {
        println!("N: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let n: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("UwU"),
        };

        let mut tri: Vec<Vec<u32>> = Vec::new();
        let mut i: usize = 0;
        while i < n + 1 {
            let mut row: Vec<u32> = Vec::new();

            let mut k: usize = 0;
            while k < i + 1 {
                let x;
                if k == 0 {
                    x = 1
                } else if k == i {
                    x = 1
                } else {
                    x = tri[i - 1][k - 1] + tri[i - 1][k]
                }

                row.push(x);
                k += 1;
            }

            tri.push(row);

            i += 1;
        }

        print!("\x1B[2J\x1B[1;1H");
        for (i, row) in tri.iter().enumerate() {
            print!("{}", " ".repeat(n - i));
            for x in row {
                print!("{} ", x);
            }
            println!("")
        }
    }
}
