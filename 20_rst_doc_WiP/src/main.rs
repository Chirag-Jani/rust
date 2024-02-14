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

    let string3 = String::from("abasdfaldcd");
    let string4 = "jaldfjad";

    let (result, result2) = longest(string1.as_str(), string2, &string3.as_str(), string4);
    println!("The longest string is: {} and {}", result, result2);
}
