// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavors {
    Chocolate,
    Vanilla,
    Citrus,
    Milk,
    Coffee,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavors,
    fluid_ounce: f32,
}

// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavors::Chocolate  => println!("Chocolate"),
        Flavors::Vanilla    => println!("Vanilla"),
        Flavors::Citrus => println!("Citrus"),
        Flavors::Milk   => println!("Milk"),
        Flavors::Coffee => println!("Coffee"),
    }

    println!("Fluid ounce: {:?}", drink.fluid_ounce);
}

fn main() {
    let drink = Drink {
        flavor: Flavors::Coffee,
        fluid_ounce: 3.3,
    };

    print_drink(drink);
}
