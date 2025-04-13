use std::io::{self, Write};

fn mod26_inverse(a: i32) -> i32 {
    for i in 1..26 {
        if (a * i) % 26 == 1 {
            return i;
        }
    }
    -1 // If no inverse exists
}

fn matrix_multiply_mod26(a: &[i32; 4], b: &[i32; 2]) -> [i32; 2] {
    [
        (a[0] * b[0] + a[1] * b[1]) % 26,
        (a[2] * b[0] + a[3] * b[1]) % 26,
    ]
}

fn mod26_matrix_inverse(matrix: &[i32; 4]) -> Option<[i32; 4]> {
    let determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    let determinant = determinant % 26;
    let inverse = mod26_inverse(determinant);
    
    if inverse == -1 {
        return None; // No inverse exists
    }

    Some([
        matrix[3] * inverse % 26,
        -matrix[1] * inverse % 26,
        -matrix[2] * inverse % 26,
        matrix[0] * inverse % 26,
    ])
}

fn encrypt(plaintext: &str, matrix: &[i32; 4]) -> String {
    let mut result = String::new();

    for chunk in plaintext.chars().collect::<Vec<_>>().chunks(2) {
        let mut block = [0; 2];
        for (i, &c) in chunk.iter().enumerate() {
            block[i] = (c.to_ascii_uppercase() as i32 - b'A' as i32) % 26;
        }

        let encrypted_block = matrix_multiply_mod26(matrix, &block);
        for &value in encrypted_block.iter() {
            result.push(((value + 26) % 26 + b'A' as i32) as u8 as char);
        }
    }

    result
}

fn decrypt(ciphertext: &str, matrix: &[i32; 4]) -> String {
    if let Some(inverse_matrix) = mod26_matrix_inverse(matrix) {
        let mut result = String::new();

        for chunk in ciphertext.chars().collect::<Vec<_>>().chunks(2) {
            let mut block = [0; 2];
            for (i, &c) in chunk.iter().enumerate() {
                block[i] = (c.to_ascii_uppercase() as i32 - b'A' as i32) % 26;
            }

            let decrypted_block = matrix_multiply_mod26(&inverse_matrix, &block);
            for &value in decrypted_block.iter() {
                result.push(((value + 26) % 26 + b'A' as i32) as u8 as char);
            }
        }

        result
    } else {
        String::from("Decryption matrix does not have an inverse.")
    }
}

fn main() {
    let mut input = String::new();
    let mut key_input = String::new();

    // Get plaintext and key
    print!("Enter plaintext (only letters): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let text = input.trim().to_uppercase();

    print!("Enter 2x2 matrix key (e.g. 6 24 1 13): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key_input).unwrap();
    let key: Vec<i32> = key_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    if key.len() != 4 {
        println!("Invalid matrix. Ensure you have exactly 4 numbers.");
        return;
    }

    let matrix: [i32; 4] = [key[0], key[1], key[2], key[3]];

    let encrypted = encrypt(&text, &matrix);
    let decrypted = decrypt(&encrypted, &matrix);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
