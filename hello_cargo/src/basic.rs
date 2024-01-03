fn main() {
    conventional_loops();
}

fn conventional_loops() {
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // it is slow
    // while index < 5 {
    //     println!("the value in whileloop is: {}", a[index]);

    //     index += 1;
    // }

    // for element in a {
    //     println!("the value in forloop is: {element}");
    // }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// fn loops() {
// type 1: normal loops
// loop {
//     println!("Again")
// }

// type 2 : assign values
// let mut counter = 0;

// let result = loop {
//     counter += 1;

//     if counter == 10 {
//         break counter * 2;
//     }
// };

//     // println!("The result is {result}");

//     // type 3 : labeled loops
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn get_val(x: u32) -> u32 {
//     x * 3
// }

// fn func_demo(x: u32) {
//     println!("{}", x)
// }

// fn tupls() { // tuples can be accessed using "." like objects in javascript
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
