// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum TicketType {
    Backstage(String),
    Vip(String),
    Standard,
}

struct Ticket {
    price: f32,
    ticket: TicketType,
}

enum TicketInfo {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        // Ticket{ price: 50.0, ticket: TicketType::Backstage("Mr.Kim".to_owned()) },
        // Ticket{ price: 65.0, ticket: TicketType::Vip("Mr.Lee".to_owned()) },
        // Ticket{ price: 30.0, ticket: TicketType::Standard },
        TicketInfo::Backstage(45.0, "Mr.Kim".to_owned()),
        TicketInfo::Vip(60.0, "Mr.Lee".to_owned()),
        TicketInfo::Standard(45.0),
    ];



    for ticket in tickets {
        match ticket {
            TicketInfo::Backstage(price, anyone) => println!("Backstage: {:?}, price: {:?}", anyone, price),
            TicketInfo::Vip(price, anyone) => println!("Vip: {:?}, price: {:?}", anyone, price),
            TicketInfo::Standard(price) => println!("Standard, price: {:?}", price),
        }
    }
}
