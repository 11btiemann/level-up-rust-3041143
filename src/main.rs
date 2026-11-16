mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut encoded_string = String::with_capacity(text.len() * 2);
        let mut last_char = '\0';
        let mut count = 0;
        for char in text.chars() {
            if char == last_char && count < 9 {
                count += 1;
            } else {
                if count > 0 {
                    encoded_string.push_str(&count.to_string());
                    encoded_string.push(last_char);
                }
                last_char = char;
                count = 1;
            }
        }
        if count > 0 {
            encoded_string.push_str(&count.to_string());
            encoded_string.push(last_char);
        }
        encoded_string
    }
    
    pub fn decode(text: &str) -> String {
        let mut decoded_string = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(count_char) = chars.next() {
            if count_char.is_ascii_digit() {
                let count = count_char.to_digit(10).unwrap();
                if let Some(ch) = chars.next() {
                    for _ in 0..count {
                        decoded_string.push(ch);
                    }
                }
            }
        }
        decoded_string
    }
}

fn main() {
    let random_text = "Hello, World XXXlarge 4222!";
    let encoded = run_length_encoding::encode(random_text);
    println!("Encoded: {} - Decoded: {}", encoded, run_length_encoding::decode(&encoded));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "Linked42";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
