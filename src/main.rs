use clap::Parser;
use colored::Colorize;
use std::i64;

const FORMAT_DEC: &str = "Dec:\t{:#}";
const FORMAT_HEX: &str = "Hex:\t{:#x}";
const FORMAT_BIN: &str = "Bin:\t{:#b}";
const FORMAT_OCT: &str = "Oct:\t{:#o}";

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
    let oct_line = format_line(FORMAT_OCT, "{:#o}", &format!("{:#o}", value));
    let bin_line = format_line(FORMAT_BIN, "{:#b}", &format!("{:#b}", value));

    if monochrome {
        println!("{}", dec_line);
        println!("{}", hex_line);
        println!("{}", oct_line);
        println!("{}", bin_line);
    } else {
        println!("{}", dec_line.bright_green());
        println!("{}", hex_line.bright_cyan());
        println!("{}", oct_line.yellow());
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
    let result = match input {
        s if s.starts_with("0x") || s.starts_with("0X") => i64::from_str_radix(&s[2..], 16),
        s if s.starts_with("0o") || s.starts_with("0O") => i64::from_str_radix(&s[2..], 8),
        s => s.parse::<i64>(),
    };

    result.map_err(|_| anyhow::anyhow!("Invalid number format"))
}
