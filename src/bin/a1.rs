// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    println!("first name: {}, last name: {}", disp_first_name(), disp_last_name());
}


fn disp_first_name() -> String {
    "MyungJin".to_string()
}


fn disp_last_name() -> String {
    "Kim".to_string()
}