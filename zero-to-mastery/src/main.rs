#[allow(dead_code)]
struct TickData {
    name: String,
    price: u32,
}

impl TickData {
    fn new(name: String, price: u32) -> Self {
        TickData { name, price }
    }

    fn print_ticket(&self) {
        println!("The Ticket holder is {} and it's price is {}.", self.name, self.price)
    }
}

enum Ticket {
    Backstage(TickData),
    Vip(TickData),
    Standard(u32),
}

fn main() {
    let mut tickets: Vec<Ticket> = Vec::new();

    tickets.push(Ticket::Backstage(TickData::new("Chirag Jani".to_string(), 999)));
    tickets.push(Ticket::Vip(TickData::new("Jon".to_string(), 899)));
    tickets.push(Ticket::Standard(499));

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(data) => data.print_ticket(),
            Ticket::Vip(data) => data.print_ticket(),
            Ticket::Standard(price) => {
                println!("This is a Standard Ticket and it's price is {}.", price);
            }
        }
    }
}
