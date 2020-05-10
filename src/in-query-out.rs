use ansi_term::Colour::Red;
use atty::Stream;
use std::io::{self, BufRead, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Query {
    #[structopt(short, long)]
    query: String,
}
fn main() {
    if atty::is(Stream::Stdin) {
        // not support for terminal input
        println!("input is terminal");
        return;
    }
    println!("input is not terminal");
    let q: Query = Query::from_args();
    println!("{:?}", q);
    let query = &q.query;
    let ansi_str = Red.paint(query);

    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());

    let stdout = io::stdout();
    let mut writter = io::BufWriter::new(stdout.lock());

    let mut buffer = String::new();
    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                if buffer.contains(query) {
                    // このあたりはより、いいやり方がありそう
                    let v: Vec<&str> = buffer.split(query).collect();
                    let mut is_first = true;
                    for elem in v {
                        if !is_first {
                            write!(writter, "{}", ansi_str).unwrap();
                        }
                        write!(writter, "{}", elem).unwrap();
                        is_first = false;
                    }
                    // こういうやり方もある
                    // write!(writter, "{}", v.join(&Red.paint(query).to_string())).unwrap();
                }
                buffer.clear();
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
