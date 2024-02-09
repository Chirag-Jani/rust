#[allow(dead_code)]
struct User {
    age: Option<f32>,
    email: String,
}

fn main() {
    let us1 = User {
        age: Some(23.0),
        email: "test@mail.com".to_owned(),
    };

    match us1.age {
        Some(age) => {
            match age {
                18.0 => println!("Teenager at age: {}", 18.0),
                _ => println!("Naahh"),
            }
        }
        None => println!("Age not provided"),
    }
}
