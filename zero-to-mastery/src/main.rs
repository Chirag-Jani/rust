#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => {
                println!("The box is blue.");
            }
            Color::Green => {
                println!("The box is green.");
            }
            Color::Red => {
                println!("The box is red.");
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Dimensions {
    length: i32,
    width: i32,
    bredth: i32,
}

impl Dimensions {
    fn square() -> Self {
        Dimensions {
            length: 10,
            width: 10,
            bredth: 10,
        }
    }
    fn print(&self) {
        println!("{:?}", self);
    }
}

struct ShippingBox {
    dimension: Dimensions,
    weight: i32,
    color: Color,
}

impl ShippingBox {
    fn new(_dimension: Dimensions, _weight: i32, _color: Color) -> Self {
        ShippingBox {
            dimension: _dimension,
            weight: _weight,
            color: _color,
        }
    }

    fn get_shipment(&self) {
        println!("Getting shipment...");
        self.dimension.print();
        println!("\nWeight  : {:?}", self.weight);
        self.color.print();
    }
}

fn main() {
    let new = ShippingBox::new(Dimensions::square(), 33, Color::Red);
    new.get_shipment();
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
