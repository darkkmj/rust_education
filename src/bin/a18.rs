// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
    name: String,
    purchase: bool,
}

fn is_available_purchase(customer: &Customer) -> Result<bool, String> {
    if customer.age >= 21 {
        Ok(true)
    }
    else {
        Err("age is under 21".to_owned())
    }
}

fn main() -> Result<(), String> {
    let customers = vec![
        Customer { age: 22, name: "Lee".to_owned(), purchase: false },
        Customer { age: 15, name: "Park".to_owned(), purchase: false },
        Customer { age: 25, name: "Kim".to_owned(), purchase: false },
    ];

    for customer in customers {
        let result = is_available_purchase(&customer)?;
        match result {
            Ok(ret) => println!("{:?}: {:?}", customer.name, ret),
            Err(ret) => println!("{:?}: {:?}", customer.name, ret)
        }
    }

    Ok(())
}
