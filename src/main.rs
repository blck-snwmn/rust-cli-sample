use std::io::{self, Write};
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");
    let param: Parameter = Parameter::from_args();
    println!("{:?}", param);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    let name = param
        .name
        .map(move |x| "-".to_string() + &x)
        .unwrap_or_default();
    let prefix = format!("{}{}", param.id, name);
    writeln!(handle, "{}:do", prefix).unwrap();
    for c in 0..param.count {
        writeln!(handle, "{}:do {}", prefix, c).unwrap();
    }
}

#[derive(Debug, StructOpt)]
struct Parameter {
    id: i64,
    #[structopt(short, long)]
    name: Option<String>,
    #[structopt(short = "t", long = "type")]
    param_type: String,
    #[structopt(short, long)]
    count: u64,
}
