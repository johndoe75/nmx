use clap::Parser;
use regex::Regex;
use std::i64;
use colored::Colorize;

const HEX_PATTERN: &str = r"^0x[0-9a-fA-F]+$";
const DEC_PATTERN: &str = r"^-?[0-9]+$";

const FORMAT_DEC: &str = "Dec:\t{:#}";
const FORMAT_HEX: &str = "Hex:\t{:#x}";
const FORMAT_BIN: &str = "Bin:\t{:#b}";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(String))]
    number: String,

    #[arg(short, long, help = "Display result without colors")]
    monochrome: bool,
}

fn main() {
    let args = Args::parse();
    let parsed_value = parse_number(&args.number).unwrap_or_else(|e| handle_parse_error(e));
    display_number_formats(parsed_value, args.monochrome);
}

fn display_number_formats(value: i64, monochrome: bool) -> () {
    let dec_line = format_line(FORMAT_DEC, "{:#}", &value.to_string());
    let hex_line = format_line(FORMAT_HEX, "{:#x}", &format!("{:#x}", value));
    let bin_line = format_line(FORMAT_BIN, "{:#b}", &format!("{:#b}", value));

    if monochrome {
        println!("{}", dec_line);
        println!("{}", hex_line);
        println!("{}", bin_line);
    } else {
        println!("{}", dec_line.bright_green());
        println!("{}", hex_line.bright_cyan());
        println!("{}", bin_line.bright_magenta());
    }
}

fn format_line(template: &str, placeholder: &str, formatted_value: &str) -> String {
    template.replace(placeholder, formatted_value)
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
