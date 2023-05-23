use std::env;
fn main() -> reqwest::Result<()> {
    let mut args = env::args();
    args.next();
    let path = match args.next() {
        Some(path) => path,
        None => String::new(),
    };

    let server_url = format!("http://127.0.0.1:7878/{path}");
    let response = reqwest::blocking::get(server_url)?;

    println!("body = {:?}", response);

    Ok(())
}
