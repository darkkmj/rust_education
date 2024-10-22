/// A favorite color
enum Color {
    Red,
    Blue,
}

/// A piece of mail
struct Main {
    address: String,
}

/// Adds two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    let choice = get_choice("mainmenu");
    println!("choice = {:?}", choice);
    pick_choice("start1");
}