fn main() {
    // let mut s = String::from("hello");

    // println!("'{}'", s);

    // takes_ownership(&mut s);

    // println!("'{}'", s);

    // let int: u32 = 22;
    // // this won't take ownership as copy is applied to int and uint
    // demo_func(int);
    // println!("'{}'", int);

    // length demo
    let str = String::from("Hello");
    println!("The string `{}` has {} bytes.", str, find_length(&str));
}

// fn takes_ownership(some_string: &mut String) {
//     some_string.push_str("    Jani   ");
//     println!("Value in function {:?}", some_string);
// }

// fn demo_func(s   ome_int: u32) {
//     // 这个函数接收一个整型参数，并返
//     println!("Some unsigned integer: {}", some_int);
// }

fn find_length(s: &String) -> usize {
    let leng = s.len();
    leng
}
