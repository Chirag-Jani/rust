use error_chain::error_chain;
use reqwest::blocking::get;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("Status : {}", res.status());
    println!("Body: \n{}", body);

    Ok(())
}