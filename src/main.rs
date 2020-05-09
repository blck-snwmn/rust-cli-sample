fn main() {
    println!("Hello, world!");
    let default = "test";
    let first_param = std::env::args().nth(1).unwrap_or(default.to_string());
    println!("{}", first_param);
}
