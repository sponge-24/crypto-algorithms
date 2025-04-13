use std::io::{self, Write};

fn caesar_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let shifted = (c as u8 - base + shift) % 26 + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn caesar_decrypt(text: &str, shift: u8) -> String {
    caesar_encrypt(text, 26 - (shift % 26))
}

fn main() {
    let mut input = String::new();

    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let text = input.trim();

    let mut shift_input = String::new();
    print!("Enter shift (0-25): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut shift_input).expect("Failed to read line");

    let shift: u8 = match shift_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid shift key. Using 0.");
            0
        }
    };

    let encrypted = caesar_encrypt(text, shift);
    let decrypted = caesar_decrypt(&encrypted, shift);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
