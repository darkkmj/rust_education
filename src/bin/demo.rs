#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: f64,
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    // match me.position {
    //     Position::Manager => println!("manager"),
    //     Position::Supervisor => println!("supervisor"),
    //     Position::Worker => println!("worker"),
    // }
    println!("{:?}", me);

    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50.0,
    };
    match concert {
        Ticket {price: 50.0, event} => println!("event @ 50 = {:?}", event),
        Ticket {price, ..} => println!("price = {:?}", price),
    }

    let response = Survey {
        q1: None,
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response"),
    }

    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        None => println!("q2: no response"),
    }

    match response.q3 {
        Some(ans) => println!("q3: {:?}", ans),
        None => println!("q3: no response"),
    }
}
