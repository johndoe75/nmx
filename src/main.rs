use clap::Parser;
use regex::Regex;
use std::i64;

const HEX_PATTERN: &str = r"^0x[0-9a-fA-F]+$";
const DEC_PATTERN: &str = r"^-?[0-9]+$";

const FORMAT_DEC: &str = "Dec:\t{:#}";
const FORMAT_HEX: &str = "Hex:\t{:#x}";
const FORMAT_BIN: &str = "Bin:\t{:#b}";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(String))]
    number: String,
}

fn main() {
    let args = Args::parse();
    let parsed_value = parse_number(&args.number).unwrap_or_else(|e| handle_parse_error(e));
    display_number_formats(parsed_value);
}

fn display_number_formats(value: i64) {
    println!("{}", FORMAT_DEC.replace("{:#}", &value.to_string()));
    println!("{}", FORMAT_HEX.replace("{:#x}", &format!("{:#x}", value)));
    println!("{}", FORMAT_BIN.replace("{:#b}", &format!("{:#b}", value)));
}

fn handle_parse_error(error: anyhow::Error) -> ! {
    eprintln!("Error: {}", error);
    std::process::exit(1);
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
