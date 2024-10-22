// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum COLOR {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
}

fn print_color(color: COLOR) {
    match color {
        COLOR::White => println!("Color is White"),
        COLOR::Black => println!("Color is Black"),
        COLOR::Red => println!("Color is Red"),
        COLOR::Green => println!("Color is Green"),
        COLOR::Blue => println!("Color is Blue"),
        COLOR::Yellow => println!("Color is Yellow"),
    }
}

fn main() {
    let color = COLOR::Red;
    print_color(color);
}
