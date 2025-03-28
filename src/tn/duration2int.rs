// made with help of chatgtp

use std::process::exit;

pub fn parse_duration(input: &str) -> i64 {
    let mut chars = input.chars().peekable();
    let mut num = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            num.push(c);
            chars.next();
        } else {
            break;
        }
    }

    let value: i64 = match num.parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: Invalid number format");
            exit(1);
        }
    };

    let unit = chars.next();

    match unit {
        Some('w') => value * 7 * 24 * 60 * 60,
        Some('d') => value * 24 * 60 * 60,
        Some('h') => value * 60 * 60,
        Some('m') => value * 60,
        Some('s') => value,
        None => value, // Treat numbers without a unit as seconds
        _ => {
            eprintln!("Error: Invalid time unit in: {}", input);
            eprintln!("expection: <number>[wdhms]");
            eprintln!("example  : 2w     for 2 weeks valid");
            eprintln!("example  : 3h     for 3 hours valid");
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("2w"), 1209600);
        assert_eq!(parse_duration("1d"), 86400);
        assert_eq!(parse_duration("3h"), 10800);
        assert_eq!(parse_duration("40m"), 2400);
        assert_eq!(parse_duration("20s"), 20);
        assert_eq!(parse_duration("1800"), 1800);
    }

    #[test]
    fn test_invalid_number() {
        let result = panic::catch_unwind(|| parse_duration("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_unit() {
        let result = panic::catch_unwind(|| parse_duration("10x"));
        assert!(result.is_err());
    }
}
