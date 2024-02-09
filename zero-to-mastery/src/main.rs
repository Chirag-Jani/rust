#[derive(Debug)]
#[allow(dead_code)]
struct Res {
    status: u8,
    msg: String,
}

fn get_data(password: String) -> Result<Res, String> {
    if password == "Jani" {
        Ok(Res { status: 200, msg: "Done".to_owned() })
    } else {
        Err("Oppsie Daiseyyy".to_owned())
    }
}

fn main() {
    let d = get_data("Chirag".to_string());
    println!("The data received is: {:?}", d)
}
