fn main() {
    loops();
}

fn loops() {
    loop {
        println!("Again")
    }
}

// fn get_val(x: u32) -> u32 {
//     x * 3
// }

// fn func_demo(x: u32) {
//     println!("{}", x)
// }

// fn tupls() {
//     // let tuple = (10, 30, 29);
//     // println!("{}", tuple.0);

//     // let x: (i32, f64, u8) = (500, 6.4, 1);
//     // println!("{}", x.0);

//     let a = [1, 23, 4, 6, 6];
//     println!("{:?}", a)
// }

// fn conversion() {
//     let guess: u32 = "42".parse().expect("Not a number!");
//     println! {"{guess}"}

//     let integer: i8 = 127;
//     // let integer = integer + 1;
//     println!("{integer}")
// }

// fn shadow_var() {
//     // scoping and SHADOWING
//     let shadow_var = 4;
//     println!("Shadow variable outside block: {}", shadow_var);

//     {
//         let shadow_var = shadow_var + 2;
//         println!("Shadow variable inside block: {}", shadow_var);
//     }

//     println!("Shadow variable again outside block: {}", shadow_var);
// }

// fn const_var() {
//     // constants
//     // mut keyword not allowed
//     // type needs to be defined like here, u32 while declaration itself
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("3 hours in seconds: {THREE_HOURS_IN_SECONDS}");
// }

// fn mut_var() {
//     // all the variables are immutable unless "mut" keyword is used while declaration
//     let mut x = 3;
//     println!("Value of x is {x}");
//     x = 34;
//     println!("New Value of x is {x}");
// }
