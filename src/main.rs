mod vigenere {
    const A: u8 = b'A';
    const Z: u8 = b'Z';
    const WRAP: u8 = 26; // ALPHABET.len() as u8

    fn clean_input(input: &str) -> impl Iterator<Item = u8> + '_ {
        input.bytes().filter_map(|x| match x {
            A ..= Z => Some(x),
            b'a' ..= b'z' => Some(x - (b'a' - A)),
            _ => None,
        })
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let mut key_iter = key.bytes().map(|k| k-A).cycle();

        let ciphertext = clean_input(plaintext)
            .map(|x| {
                let offset = key_iter.next().unwrap();
                ((x - A + offset) % WRAP) + A
            })
            .collect();

        String::from_utf8(ciphertext).unwrap()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut key_iter = key.bytes().map(|k| k-A).cycle();

        let ciphertext = clean_input(ciphertext)
            .map(|x| {
                let offset = key_iter.next().unwrap();
                ((x + WRAP -A) - offset) % WRAP + A
            })
            .collect();

        String::from_utf8(ciphertext).unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::{decrypt, encrypt};

        #[test]
        fn encrypts_with_vigenere_vector() {
            assert_eq!(encrypt("ATTACKATDAWN", "LEMON"), "LXFOPVEFRNHR");
        }

        #[test]
        fn decrypts_encrypted_text() {
            let plaintext = "YOUROCK";
            let key = "WHYRUST";

            assert_eq!(decrypt(&encrypt(plaintext, key), key), plaintext);
        }
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("Encrypted: {}, Plaintext: {}", ciphertext, plaintext);

    let message = "YOUROCK";
    println!("Plaintext: {}, Encrypted: {}, Decrypted again: {}",
        message, vigenere::encrypt(message, key),
        vigenere::decrypt(&vigenere::encrypt(message, key), key));
}
