use std::io::{self, Write};

fn august_encrypt(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                ((c as u8 - base + 1) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}

fn august_decrypt(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                ((c as u8 - base + 25) % 26 + base) as char // equivalent to -1
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let mut input = String::new();

    print!("Enter text to encrypt with August Cipher: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let text = input.trim();
    let encrypted = august_encrypt(text);
    let decrypted = august_decrypt(&encrypted);

    println!("\nEncrypted (August Cipher): {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
