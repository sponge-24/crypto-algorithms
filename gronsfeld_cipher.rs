use std::io::{self, Write};

fn gronsfeld_encrypt(plaintext: &str, key: &str) -> String {
    let key_digits: Vec<u8> = key
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

    let mut key_index = 0;

    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();
                let base = if is_upper { b'A' } else { b'a' };
                let p = c as u8 - base;
                let k = key_digits[key_index % key_digits.len()];
                key_index += 1;
                ((p + k) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}

fn gronsfeld_decrypt(ciphertext: &str, key: &str) -> String {
    let key_digits: Vec<u8> = key
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

    let mut key_index = 0;

    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();
                let base = if is_upper { b'A' } else { b'a' };
                let p = c as u8 - base;
                let k = key_digits[key_index % key_digits.len()];
                key_index += 1;
                ((p + 26 - k) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    let mut key_input = String::new();

    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let text = input.trim();

    print!("Enter numeric key (digits only): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key_input).unwrap();
    let key = key_input.trim();

    if key.is_empty() || !key.chars().all(|c| c.is_ascii_digit()) {
        println!("Invalid key. It must be a non-empty numeric string (0–9).");
        return;
    }

    let encrypted = gronsfeld_encrypt(text, key);
    let decrypted = gronsfeld_decrypt(&encrypted, key);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
