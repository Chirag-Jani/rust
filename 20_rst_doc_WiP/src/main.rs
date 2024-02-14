use std::result;

fn longest<'a, 'zzz>(x: &'a str, y: &'a str, w: &'zzz str, z: &'zzz str) -> (&'a str, &'zzz str) {
    let r1;
    let r2;

    if x.len() > y.len() {
        r1 = x;
    } else {
        r1 = y;
    }

    if w.len() > z.len() {
        r2 = w;
    } else {
        r2 = z;
    }

    (r1, r2)
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let string4 = "this is longer than string3";

    let result;
    let result2;

    {
        let string3 = String::from("abasdfaldcd");
        (result, result2) = longest(string1.as_str(), string2, &string3.as_str(), string4);
        // the smallest lifetime is assigned and thus this will give us below error
        // `string3` does not live long enough
        // borrowed value does not live long enoughrustcClick for full compiler diagnostic
        // main.rs(35, 5): `string3` dropped here while still borrowed
        // main.rs(32, 13): binding `string3` declared here
        // main.rs(37, 58): borrow later used here
    }

    println!("The longest string is: {} and {}", result, result2);
}
