use atty::Stream;
use std::io::{self, BufRead, Write};
fn main() {
    if atty::is(Stream::Stdin) {
        // not support for terminal input
        println!("input is terminal")
    }
    println!("input is not terminal");
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());

    let stdout = io::stdout();
    let mut writter = io::BufWriter::new(stdout.lock());

    let mut buffer = String::new();
    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                write!(writter, "{}", buffer).unwrap();
                buffer.clear();
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
