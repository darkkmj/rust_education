// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(gt_100: bool) {
    println!("{:?}", match gt_100 {
        true => "its big",
        _ => "its small",
    });
}

fn main() {
    let variable = 90;
    // let result = variable > 100;
    let result = if variable > 100 {
            true
        }
        else {
            false
        };

    print_message(result);
}

