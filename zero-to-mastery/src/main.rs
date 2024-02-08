fn main() {
    // first approach
    // let numbers = vec![10, 20, 30, 40];

    // another way to do
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    numbers.push(40);

    for n in &numbers {
        // first approach
        // if n == 30 {
        //     println!("Thirty");
        // } else {
        //     println!("{}", n);
        // }

        // could have used match
        match n {
            30 => println!("Thirty"),
            _ => println!("{}", n),
        }
    }

    println!("Total elements in your vector: {}", numbers.len());
}
