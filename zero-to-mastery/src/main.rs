#[allow(dead_code)]
#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: String, age: u8) -> Result<Adult, String> {
        if age > 21 {
            Ok(Self { name, age })
        } else {
            Err("Not an Adult as AGe is < 21".to_string())
        }
    }
}

fn main() {
    let mut members: Vec<Adult> = Vec::new();

    let ad1 = Adult::new("Chirag".to_owned(), 10);
    let ad2 = Adult::new("Jani".to_owned(), 100);

    match ad1 {
        Ok(adult) => {
            println!("{:?}", adult);
            members.push(adult);
        }
        Err(reason) => println!("{:?}", reason),
    }

    match ad2 {
        Ok(adult) => {
            println!("{:?}", adult);
            members.push(adult);
        }
        Err(reason) => println!("{:?}", reason),
    }

    println!("The final list of adults is: {:?}", members)
}
