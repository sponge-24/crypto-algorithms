use std::io::{self, Write};

fn atbash_cipher(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let reversed = base + (25 - (c as u8 - base));
                reversed as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let mut input = String::new();

    print!("Enter text for Atbash cipher: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let text = input.trim();
    let encrypted = atbash_cipher(text);

    println!("Atbash Cipher Output: {}", encrypted);
}
