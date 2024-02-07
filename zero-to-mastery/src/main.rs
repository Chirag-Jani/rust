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
}

fn main() {
    let demo = GroceryItem { id: 9387654, quantity: 43 };
    demo.print_id();
    GroceryItem::print_quantity(&demo)
}
