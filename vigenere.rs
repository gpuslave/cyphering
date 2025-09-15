
//! Implements the Vigenère cipher.

/// Shifts a character `c` using a character from the key `key_char`.
/// `direction` is 1 for encryption and -1 for decryption.
fn shift(c: char, key_char: char, direction: i32) -> char {
    const ALPHABET_LEN: i32 = 26;
    // This implementation only works for uppercase English letters.
    if c.is_ascii_uppercase() && key_char.is_ascii_uppercase() {
        let c_val = c as i32 - 'A' as i32;
        let key_val = key_char as i32 - 'A' as i32;

        // The core formula: (Value + (direction * Key)) mod 26
        let new_val = (c_val + direction * key_val + ALPHABET_LEN) % ALPHABET_LEN;

        // Convert the new value back to a character.
        (new_val as u8 + b'A') as char
    } else {
        // If the character is not an uppercase letter, return it unchanged.
        c
    }
}

/// Encrypts text using the Vigenère cipher.
/// Non-alphabetic characters are passed through unchanged.
pub fn encrypt(plaintext: &str, key: &str) -> String {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Warning: Key must be non-empty and contain only alphabetic characters.");
        return plaintext.to_string();
    }

    // Create an iterator that cycles through the key's characters, converted to uppercase.
    let mut key_chars = key.chars().map(|c| c.to_ascii_uppercase()).cycle();

    plaintext
        .chars()
        .map(|c| {
            // The key iterator only advances when we encrypt an alphabetic character.
            if c.is_ascii_alphabetic() {
                shift(c.to_ascii_uppercase(), key_chars.next().unwrap(), 1)
            } else {
                c
            }
        })
        .collect()
}

/// Decrypts text encrypted with the Vigenère cipher.
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Warning: Key must be non-empty and contain only alphabetic characters.");
        return ciphertext.to_string();
    }

    let mut key_chars = key.chars().map(|c| c.to_ascii_uppercase()).cycle();

    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                shift(c.to_ascii_uppercase(), key_chars.next().unwrap(), -1)
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let key = "AMEBA";
    let plaintext = "SHIFROVANIE";

    println!("Plaintext:  {}", plaintext);
    println!("Key:        {}", key);

    let encrypted_text = encrypt(plaintext, key);
    println!("Encrypted:  {}", encrypted_text);

    let decrypted_text = decrypt(&encrypted_text, key);
    println!("Decrypted:  {}", decrypted_text);

    assert_eq!(plaintext, decrypted_text);
    println!("Encryption and decryption successful!");

//     let long_text = "THIS IS A LONGER TEXT FOR A MORE COMPLETE DEMONSTRATION OF THE VIGENERE CIPHER.";
//     let encrypted_long = encrypt(long_text, key);
//     println!("
// Original long text:  {}", long_text);
//     println!("Encrypted long text: {}", encrypted_long);
//     println!("Decrypted long text: {}", decrypt(&encrypted_long, key));
}
