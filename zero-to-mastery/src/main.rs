#[allow(dead_code)]
struct User {
    age: Option<f32>,
    email: String,
}

impl User {
    fn new(age: f32, email: String) -> Self {
        User {
            age: Some(age),
            email,
        }
    }
}

fn main() {
    let mut users: Vec<User> = Vec::new();

    let us1 = User {
        age: Some(23.0),
        email: "test@mail.com".to_owned(),
    };

    users.push(us1);
    users.push(User::new(45.0, "email@example.com".to_string()));
    users.push(User { age: None, email: "hehe@heeh.com".to_owned() });

    for user in users {
        match user {
            User { age, email } => {
                println!("Age is: {:?} and Mail is : {:}", age, email);
            }
        }
    }
}
