use std::io::{self, Write};

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Some(x);
        }
    }
    None
}

fn affine_encrypt(text: &str, a: i32, b: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let x = (c as u8 - base) as i32;
                let encrypted = (a * x + b) % 26;
                (base + encrypted as u8) as char
            } else {
                c
            }
        })
        .collect()
}

fn affine_decrypt(text: &str, a: i32, b: i32) -> String {
    let a_inv = mod_inverse(a, 26).expect("a and 26 must be coprime");
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let y = (c as u8 - base) as i32;
                let decrypted = (a_inv * (y - b + 26)) % 26;
                (base + decrypted as u8) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    let mut a_input = String::new();
    let mut b_input = String::new();

    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let text = input.trim();

    print!("Enter key a (must be coprime with 26): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a_input).unwrap();
    let a: i32 = a_input.trim().parse().unwrap_or(1);

    print!("Enter key b: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b_input).unwrap();
    let b: i32 = b_input.trim().parse().unwrap_or(0);

    if mod_inverse(a, 26).is_none() {
        println!("Invalid key 'a'. It must be coprime with 26.");
        return;
    }

    let encrypted = affine_encrypt(text, a, b);
    let decrypted = affine_decrypt(&encrypted, a, b);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
