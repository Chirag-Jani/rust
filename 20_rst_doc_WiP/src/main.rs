use std::fmt::Display;

fn longest<'a, GenericType>(x: &'a str, y: &'a str, name: GenericType) -> &'a str
where
    GenericType: Display,
{
    println!("This function is called by {name}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result;

    (result) = longest(string1.as_str(), string2, "User2");

    println!("The longest string is: {}", result);
}
