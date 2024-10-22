// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity:i32,
    id:i32,
}

impl GroceryItem {
    fn init() -> Self {
        Self {
            quantity: 30,
            id: 22,
        }
    }

    fn disp_quantity(&self) {
        println!("quantity: {:?}", self.quantity)
    }
    
    fn disp_id(&self) {
        println!("id: {:?}", self.id)
    }
}

fn main() {
    let item = GroceryItem {
        id: 11,
        quantity: 25,
    };

    let second_item = GroceryItem::init();

    item.disp_quantity();
    item.disp_id();

    second_item.disp_quantity();
    second_item.disp_id();
}
