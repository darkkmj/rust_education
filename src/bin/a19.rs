// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut items = HashMap::new();

    items.insert("Chair", 5);
    items.insert("Bed", 3);
    items.insert("Table", 2);
    items.insert("Couch", 0);

    let mut total_stock = 0;
    for (item, number) in items.iter() {
        if number == &0 {
            println!("{:?}: out of stock", item);
        }
        else {
            println!("{:?}: {:?}", item, number);
        }

        total_stock += number;
    }

    println!("total stock items: {:?}", total_stock);
}
