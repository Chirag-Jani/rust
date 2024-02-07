struct GroceryItem {
    id: i32,
    quantity: i32,
}

impl GroceryItem {
    fn print_id(&self) {
        println!("ID: {}", self.id);
    }
    fn print_quantity(data: &GroceryItem) {
        println!("Quantity: {}", data.quantity);
    }
    fn new_item() -> Self {
        Self {
            id: 0,
            quantity: 0,
        }
    }
}

fn main() {
    let demo = GroceryItem { id: 9387654, quantity: 43 };
    demo.print_id();
    GroceryItem::print_quantity(&demo);

    let demo2 = GroceryItem::new_item();
    demo2.print_id();
}

/*
- variables & data types
- if else
- match case
- loops (for-loop & loop keyword, while revise)
- functions
- enums
- tuple (skipped, feels easy)
- structs
- impl (basic)
- ownership (borrows)
*/
