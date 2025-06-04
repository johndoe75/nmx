use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(i32))]
    number: i32,
}

fn main() {
    let args = Args::parse();
    println!("Dec:\t{:#}", args.number);
    println!("Hex:\t{:#x}", args.number);
    println!("Bin:\t{:#b}", args.number);
}
