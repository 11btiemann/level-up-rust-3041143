use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnErr {
    TooShort,
    TooLong,
    WrongCheckDigit,
    IllegalCharacter
}

impl FromStr for Isbn {
    type Err = IsbnErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let no_hyphens = s.replace("-", "");
        if !no_hyphens.chars().all(|c| c.is_ascii_digit()) { 
            return Err(IsbnErr::IllegalCharacter) 
        }
        if no_hyphens.len() < 13 { return Err(IsbnErr::TooShort) };
        if no_hyphens.len() > 13 { return Err(IsbnErr::TooLong) };

        let digits: Vec<u8> = no_hyphens.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        if calculate_check_digit(&digits[..12]) != digits[12] {
            return Err(IsbnErr::WrongCheckDigit);
        }

        Ok(Isbn { raw: s.to_string(), digits })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let sum = digits.iter().enumerate().map(|(i, &x)| if i % 2 == 0 { x } else { x * 3 }).sum::<u8>();
    (10 - (sum % 10)) % 10
}

fn main() {
    match "978-3-16-148410-0".parse::<Isbn>() {
        Ok(rust_in_action) => println!("Rust in Action's ISBN-13 ({})is valid, digits: {}!",
            rust_in_action, 
            rust_in_action.digits.iter().map(|d| (d + b'0') as char).collect::<String>()),
        Err(e) => println!("Error parsing ISBN: {:?}", e),
    }
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}

#[test]
fn test_for_illegal_character() {
    let result: Result<Isbn, _> = "978-x-16-148410-0".parse();
    assert!(result.is_err());
    match result {
        Err(IsbnErr::IllegalCharacter) => (),
        _ => panic!("Expected IllegalCharacter error"),
    }
}
