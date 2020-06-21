mod config;

fn main() {
    let config = config::parse_args();

    println!("Hello, world!");
    println!("{:?}", config);
}
