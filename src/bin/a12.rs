// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

use std::fmt::Display;

enum BoxColor {
    Green,
    Red,
    Blue,
    Black,
    White,
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

struct BoxCharacteristics {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor
}

impl Display for BoxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxColor::Green => write!(f, "color is Green"),
            BoxColor::Red => write!(f, "color is Red"),
            BoxColor::Blue => write!(f, "color is Blue"),
            BoxColor::Black => write!(f, "color is Black"),
            BoxColor::White => write!(f, "color is White"),
        }
        
    }
}

impl Dimensions {
    fn print(&self) {
        println!("  width: {:?}", self.width);
        println!("  height: {:?}", self.height);
        println!("  depth: {:?}", self.depth);
    } 
}

impl BoxCharacteristics {
    fn new(weight: f64, color: BoxColor, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn disp(&self) {
        println!("Box characteristics");
        println!("  {}", self.color);
        self.dimensions.print();
        println!("  weight: {:?}", self.weight);
    }
}


fn main() {
    let shipping_box: BoxCharacteristics = BoxCharacteristics::new(5.0, BoxColor::Red, Dimensions { width: 1.0, height: 2.0, depth: 3.0, });
    shipping_box.disp();
}
