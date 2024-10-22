// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.
use std::collections::HashMap;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    bill_list: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            bill_list: HashMap::new()
        }
    }

    fn add(&mut self, name: String, amount: f64) {

    }

    fn view(self) {
        for bill in self.bill_list {
            println!("{bill}");
        }
    }

    fn remove(&mut self, name: String) {
        self.bill_list.remove(name.as_str());
    }

    fn edit(&mut self, name: String, amount: f64) {

    }
}

enum Command {
    Add,
    View,
    Remove,
    Edit,
}

fn print_menu() {
    println!("1. Add");
    println!("2. View");
    println!("3. Remove");
    println!("4. Edit");
    println!("Enter your command: ");
}

fn get_name() -> String {
    
}

fn main() {
    let mut bill_list = Bills::new();
    let mut buffer = String::new();
    loop {
        print_menu();
        let input_cmd = std::io::stdin().read_line(&mut buffer);
        match input_cmd {
            None => {}
            Some("_") => {}
        }
    }
}
