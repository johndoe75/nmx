use clap::Parser;
use regex::Regex;
use std::i64;

const HEX_PATTERN: &str = r"^0x[0-9a-fA-F]+$";
const DEC_PATTERN: &str = r"^-?[0-9]+$";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(String))]
    number: String,
}

fn main() {
    let args = Args::parse();

    let parsed_value = match parse_number(&args.number) {
        Ok(number) => number,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        },
    };

    println!("Dec:\t{:#}", parsed_value);
    println!("Hex:\t{:#x}", parsed_value);
    println!("Bin:\t{:#b}", parsed_value);
}

fn parse_number(input: &str) -> anyhow::Result<i64> {
    let hex_reg_ex = Regex::new(HEX_PATTERN)?;
    let dec_reg_ex = Regex::new(DEC_PATTERN)?;

    if !hex_reg_ex.is_match(input) && !dec_reg_ex.is_match(input) {
        anyhow::bail!("Invalid number format: {}", input);
    }

    if hex_reg_ex.is_match(input) {
        Ok(i64::from_str_radix(&input.trim_start_matches("0x"), 16)?)
    } else {
        Ok(input.parse::<i64>()?)
    }
}
