use std::io::{self, Write};

fn vigenere_encrypt(plaintext: &str, key: &str) -> String {
    let upper_key = key.to_uppercase();
    let key_bytes = upper_key.as_bytes();
    let mut key_index = 0;

    plaintext.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();
                let base = if is_upper { b'A' } else { b'a' };
                let p = c as u8 - base;
                let k = key_bytes[key_index % key_bytes.len()] - b'A';
                key_index += 1;
                ((p + k) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}

fn vigenere_decrypt(ciphertext: &str, key: &str) -> String {
    let upper_key = key.to_uppercase();
    let key_bytes = upper_key.as_bytes();
    let mut key_index = 0;

    ciphertext.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();
                let base = if is_upper { b'A' } else { b'a' };
                let p = c as u8 - base;
                let k = key_bytes[key_index % key_bytes.len()] - b'A';
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

    print!("Enter keyword: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key_input).unwrap();
    let key = key_input.trim();

    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Invalid key. It must be a non-empty alphabetic string.");
        return;
    }

    let encrypted = vigenere_encrypt(text, key);
    let decrypted = vigenere_decrypt(&encrypted, key);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
