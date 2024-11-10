// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

use std::fmt;

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesItem (Color);

#[derive(Debug)]
struct ShirtItem (Color);

#[derive(Debug)]
struct PantsItem (Color);

impl ShoesItem {
    fn new(color: Color) -> Self {
        Self(color)
    }

    fn print_data(self) {
        println!("shoes item color: {:?}", self.0)
    }
}

impl ShirtItem {
    fn new(color: Color) -> Self {
        Self(color)
    }

    fn print_data(self) {
        println!("shirt item color: {:?}", self.0)
    }
}

impl PantsItem {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Blue => Ok(Self(color)),
            _ => Err("this color is not allowed".to_owned())
        }
    }

    fn print_data(self) {
        println!("pants item color: {:?}", self.0)
    }
}

fn main() {
    let shoes_item = ShoesItem::new(Color::Blue);
    let shirt_item = ShirtItem::new(Color::Brown);
    let pants_item = PantsItem::new(Color::Gray);

    shoes_item.print_data();
    shirt_item.print_data();
    if pants_item.is_ok() {
        pants_item.unwrap().print_data();
    }
    else {
        println!("pants item error: {}", pants_item.unwrap_err());
    }
}
