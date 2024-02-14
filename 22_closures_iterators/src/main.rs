#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    RED,
    BLUE,
    PINK,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtSize {
    M,
    L,
    XL,
    XXL,
}

#[derive(Debug)]
struct Shirt {
    color: ShirtColor,
    size: ShirtSize,
}

impl Shirt {
    fn new(color: ShirtColor, size: ShirtSize) -> Shirt {
        Self { color, size }
    }
}

#[derive(Debug)]
struct Inventory {
    stock: Vec<Shirt>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_stock = 0;
        let mut blue_stock = 0;
        let mut pink_stock = 0;

        for shirt in &self.stock {
            match shirt.color {
                ShirtColor::RED => red_stock += 1,
                ShirtColor::BLUE => blue_stock += 1,
                ShirtColor::PINK => pink_stock += 1,
            }
        }

        if red_stock > blue_stock {
            if red_stock > pink_stock {
                ShirtColor::RED
            } else {
                ShirtColor::PINK
            }
        } else {
            if blue_stock > pink_stock {
                ShirtColor::BLUE
            } else {
                ShirtColor::PINK
            }
        }
    }
}

fn main() {
    let inventory = Inventory {
        stock: vec![
            Shirt::new(ShirtColor::RED, ShirtSize::XL),
            Shirt::new(ShirtColor::BLUE, ShirtSize::XXL),
            Shirt::new(ShirtColor::BLUE, ShirtSize::L),
            Shirt::new(ShirtColor::PINK, ShirtSize::M),
        ],
    };

    let user_pref1 = ShirtColor::RED;
    let giveaway1 = inventory.giveaway(Some(user_pref1));
    println!(
        "User 1 with preferance {:?} got: {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = Option::None;
    let giveaway2 = inventory.giveaway(user_pref2); // passing none here
    println!(
        "User 2 with preferance {:?} got: {:?}",
        user_pref2, giveaway2
    );
}
