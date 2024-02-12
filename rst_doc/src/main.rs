#[derive(Debug)]
#[allow(dead_code)]
struct Demo<A, B, C> {
    age: A,
    name: B,
    nums: C,
}

impl<A, B, C> Demo<A, B, C> {
    fn new(age: A, name: B, nums: C) -> Self {
        Demo { age, name, nums }
    }
}

fn main() {
    let data= Demo::new("Hehe", "Hihi", [1,2,3,4,5]);

    println!("{:?}", data);
}
