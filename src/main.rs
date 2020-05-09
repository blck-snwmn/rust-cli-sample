use structopt::StructOpt;

fn main() {
    println!("Hello, world!");
    // let default = "test";
    // let first_param = std::env::args().nth(1).unwrap_or(default.to_string());
    // println!("{}", first_param);
    let param = Parameter::from_args();
    println!("{:?}", param);
}

#[derive(Debug, StructOpt)]
struct Parameter {
    id: i64,
    #[structopt(short, long)]
    name: Option<String>,
    #[structopt(short = "t", long = "type")]
    param_type: String,
}
