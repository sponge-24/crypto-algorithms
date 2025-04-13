use std::io::{self, Write};

fn rail_fence_encrypt(plaintext: &str, rails: usize) -> String {
    let mut fence = vec![vec![' '; plaintext.len()]; rails];
    let mut row = 0;
    let mut down = true;

    // Fill the fence in a zigzag pattern
    for (i, c) in plaintext.chars().enumerate() {
        fence[row][i] = c;
        if down {
            if row < rails - 1 {
                row += 1;
            } else {
                down = false;
                row -= 1;
            }
        } else {
            if row > 0 {
                row -= 1;
            } else {
                down = true;
                row += 1;
            }
        }
    }

    // Read the ciphertext row by row
    let mut ciphertext = String::new();
    for r in 0..rails {
        for c in 0..plaintext.len() {
            if fence[r][c] != ' ' {
                ciphertext.push(fence[r][c]);
            }
        }
    }

    ciphertext
}

fn rail_fence_decrypt(ciphertext: &str, rails: usize) -> String {
    let mut fence = vec![vec![' '; ciphertext.len()]; rails];
    let mut row = 0;
    let mut down = true;
    let mut ciphertext_chars = ciphertext.chars();

    // Mark the positions in the fence
    for i in 0..ciphertext.len() {
        fence[row][i] = '*';
        if down {
            if row < rails - 1 {
                row += 1;
            } else {
                down = false;
                row -= 1;
            }
        } else {
            if row > 0 {
                row -= 1;
            } else {
                down = true;
                row += 1;
            }
        }
    }

    // Fill the fence with the ciphertext characters
    for r in 0..rails {
        for c in 0..ciphertext.len() {
            if fence[r][c] == '*' {
                fence[r][c] = ciphertext_chars.next().unwrap();
            }
        }
    }

    // Read the plaintext row by row
    let mut plaintext = String::new();
    row = 0;
    down = true;
    for i in 0..ciphertext.len() {
        plaintext.push(fence[row][i]);
        if down {
            if row < rails - 1 {
                row += 1;
            } else {
                down = false;
                row -= 1;
            }
        } else {
            if row > 0 {
                row -= 1;
            } else {
                down = true;
                row += 1;
            }
        }
    }

    plaintext
}

fn main() {
    let mut input = String::new();
    let mut key_input = String::new();

    print!("Enter plaintext (only letters): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let text = input.trim();

    print!("Enter number of rails: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key_input).unwrap();
    let rails: usize = key_input.trim().parse().unwrap();

    let encrypted = rail_fence_encrypt(text, rails);
    let decrypted = rail_fence_decrypt(&encrypted, rails);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
