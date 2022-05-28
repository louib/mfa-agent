use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};

use std::convert::{TryInto, TryFrom, From};


// We need a separator that is not used with base64 encoding.
pub const NONCE_SEPARATOR: &str = ":";
// This nonce size is required by the aes_gcm library.
pub const NONCE_SIZE: usize = 8;
// This key size is required by the aes_gcm library.
pub const KEY_SIZE: usize = 32;

pub fn encrypt(plaintext: &str, key: &str) -> String {
    if key.len() != KEY_SIZE {
        panic!("Invalid key length {}", key.len());
    }

    let nonce: [u8; NONCE_SIZE] = rand::random::<[u8; NONCE_SIZE]>();
    let nonce = base64::encode(&nonce);

    let plaintext = base64::encode(plaintext);

    let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_bytes()));
    let ciphertext = match cipher.encrypt(
        GenericArray::from_slice(nonce.as_bytes().as_ref()),
        plaintext.as_bytes().as_ref()
    ) {
        Ok(encrypted) => encrypted,
        Err(_) => return String::from(""),
    };
    let ciphertext = base64::encode(&ciphertext);

    format!("{}{}{}", ciphertext, NONCE_SEPARATOR, &nonce)
}

pub fn decrypt(cipher_text: &str, key: &str) -> String {
    if key.len() != KEY_SIZE {
        panic!("Invalid key length {}", key.len());
    }

    // TODO return an Err instead.
    if ! cipher_text.contains(NONCE_SEPARATOR) {
        panic!("Invalid ciphertext {}", cipher_text);
    }

    let parts: Vec<&str> = cipher_text.split(NONCE_SEPARATOR).collect();
    // TODO return an Err instead.
    if parts.len() != 2 {
        panic!("Invalid ciphertext {}", cipher_text);
    }

    let ciphertext = parts[0];
    let ciphertext = base64::decode(ciphertext).unwrap();
    let nonce = parts[1];

    // eprintln!("ciphertext is {}", ciphertext);
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_bytes()));
    let plaintext = match cipher.decrypt(
        GenericArray::from_slice(nonce.as_bytes().as_ref()),
        ciphertext.as_ref()
    ) {
        Ok(plaintext) => plaintext,
        Err(_) => panic!("Could not decrypt ciphertext"),
    };
    let plaintext = base64::decode(&plaintext).unwrap();

    return String::from_utf8(plaintext).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: &str = "super-secret-key-fdsfsdfddfddddd";

    #[test]
    pub fn test_encrypt() {
        let plain_text = "allooooooo";
        let encrypted_string = encrypt(plain_text, TEST_KEY);
        assert!(!encrypted_string.is_empty());
        assert!(encrypted_string.contains(NONCE_SEPARATOR));
    }

    #[test]
    #[should_panic]
    pub fn test_missing_separator() {
        decrypt("invalid", TEST_KEY);
    }

    #[test]
    #[should_panic]
    pub fn test_invalid_ciphertext() {
        decrypt("invalid:ciphertext:ok?", TEST_KEY);
    }

    #[test]
    pub fn test_decrypt() {
        let cipher_text = "/kOLv7RYVgpbnWtDp1XQXMT7+VhsjLFIhabRqgz/AUY=:xc5xHJiBkVE=";
        let plain_text = decrypt(cipher_text, TEST_KEY);
        assert_eq!("allooooooo", plain_text);
    }

    #[test]
    #[should_panic]
    pub fn test_decrypt_invalid_key() {
        let cipher_text = "/kOLv7RYVgpbnWtDp1XQXMT7+VhsjLFIhabRqgz/AUY=:xc5xHJiBkVE=";
        let plain_text = decrypt(cipher_text, "super-secret-key-ddddddddddddddd");
        assert_eq!("allooooooo", plain_text);
    }

    #[test]
    #[should_panic(expected="Invalid key length")]
    pub fn test_decrypt_invalid_key_length() {
        let cipher_text = "/kOLv7RYVgpbnWtDp1XQXMT7+VhsjLFIhabRqgz/AUY=:xc5xHJiBkVE=";
        let plain_text = decrypt(cipher_text, "key-with-invalid-length");
        assert_eq!("allooooooo", plain_text);
    }
}
