pub mod department_tui {
    use std::io;

    /*
    Using a hash map and vectors, create a text interface to allow a user to add employee names
    to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    Then let the user retrieve a list of all people in a department or all people in the company
    by department, sorted alphabetically.
    */

    #[derive(Debug)]
    struct Employee {
        name: String,
        department: String,
    }

    #[allow(dead_code)]
    pub fn run() {
        let mut input_str = String::new();

        let mut employees: Vec<Employee> = Vec::new();
        loop {
            println!("Who do you want to assign to which department? (Add <name> to <department>)");

            input_str.clear();
            io::stdin()
                .read_line(&mut input_str)
                .expect("Failed to read line");

            if input_str.trim().is_empty() {
                break;
            }

            //Add <name> to <department>
            input_str = input_str.replace(".", "");
            let mut words = input_str.split_whitespace();
            words.next();
            let name = String::from(words.next().expect("Input too short"));
            words.next();
            let department = String::from(words.next().expect("Input too short"));
            employees.push(Employee { name, department });
        }

        println!("Name       | Department");
        println!("-----------|-----------");
        for employee in employees {
            println!(
                "{} | {}",
                pad_str(employee.name, 10),
                pad_str(employee.department, 10)
            );
        }
        //print!("\x1B[2J\x1B[1;1H");
    }

    fn pad_str(mut str: String, length: usize) -> String {
        str.push_str(" ".repeat(length - str.len()).as_str());
        str
    }
}
