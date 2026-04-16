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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_number("42").unwrap(), 42);
        assert_eq!(parse_number("0").unwrap(), 0);
        assert_eq!(parse_number("-10").unwrap(), -10);
    }

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_number("0xFF").unwrap(), 255);
        assert_eq!(parse_number("0X1a").unwrap(), 26);
        assert_eq!(parse_number("0x0").unwrap(), 0);
    }

    #[test]
    fn test_parse_octal() {
        assert_eq!(parse_number("0o77").unwrap(), 63);
        assert_eq!(parse_number("0O10").unwrap(), 8);
        assert_eq!(parse_number("0o0").unwrap(), 0);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_number("abc").is_err());
        assert!(parse_number("1.5").is_err());
        assert!(parse_number("").is_err());
    }

    #[test]
    fn test_format_line() {
        assert_eq!(format_line("Dec: {:#}", "{:#}", "42"), "Dec: 42");
        assert_eq!(format_line("Hex: {:#x}", "{:#x}", "ff"), "Hex: ff");
    }

    #[test]
    fn test_edge_cases_max_min() {
        assert_eq!(parse_number("9223372036854775807").unwrap(), i64::MAX);
        assert_eq!(parse_number("-9223372036854775808").unwrap(), i64::MIN);
    }

    #[test]
    fn test_edge_cases_leading_zeros() {
        assert_eq!(parse_number("007").unwrap(), 7);
        assert_eq!(parse_number("0x00FF").unwrap(), 255);
        assert_eq!(parse_number("0o0077").unwrap(), 63);
    }

    #[test]
    fn test_edge_cases_mixed_case() {
        assert_eq!(parse_number("0xFf").unwrap(), 255);
        assert_eq!(parse_number("0XaA").unwrap(), 170);
        assert_eq!(parse_number("0O77").unwrap(), 63);
    }

    #[test]
    fn test_edge_cases_large_hex() {
        assert_eq!(parse_number("0xFFFFFFFF").unwrap(), 4294967295_i64);
        assert!(parse_number("0xFFFFFFFFFFFFFFFF").is_err());
    }

    #[test]
    fn test_edge_cases_overflow() {
        assert!(parse_number("9223372036854775808").is_err());
        assert!(parse_number("-9223372036854775809").is_err());
    }

    #[test]
    fn test_edge_cases_invalid_prefix() {
        assert!(parse_number("0b101").is_err());
        assert!(parse_number("0x").is_err());
        assert!(parse_number("0o").is_err());
        assert!(parse_number("0xGG").is_err());
        assert!(parse_number("0o89").is_err());
    }
}
