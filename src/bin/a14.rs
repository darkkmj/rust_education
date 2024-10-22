// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct PersonInfo {
    age: i32,
    name: String,
    color: String,
}

fn print_info(name: &str, color: &str) {
    println!("name: {:?}, color: {:?}", name, color);
}

fn main() {
    let persons_info = vec![
        PersonInfo {
            age: 8,
            name: "kim".to_owned(),
            color: "red".to_owned()
        },
        PersonInfo {
            age: 31,
            name: "lee".to_owned(),
            color: "blue".to_owned()
        },
        PersonInfo {
            age: 6,
            name: "park".to_owned(),
            color: "yellow".to_owned()
        },
    ];

    for info in persons_info {
        if info.age <= 10 {
            print!("age: {:?}, ", info.age);
            print_info(&info.name, &info.color);
        }
    }
}
