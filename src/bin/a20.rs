// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        use PowerState::*;
        match state.trim().to_lowercase().as_str() {
            "off" => Some(Off),
            "sleep" => Some(Sleep),
            "Reboot" => Some(Reboot),
            "Shutdown" => Some(Shutdown),
            "Hibernate" => Some(Hibernate),
            _ => None,
        }
    }
}

fn print_power_state(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("Turning off"),
        Sleep => println!("Sleeping"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
    let mut buffer = String::new();
    let user_input = io::stdin().read_line(&mut buffer);
    if user_input.is_ok() {
        let state = PowerState::new(&buffer);
        match state {
            Some(input) => print_power_state(input),
            None => println!("Invalid input type"),
        }
    }
    else {
        println!("Invalid input error {:?}", user_input.err());
    }
}