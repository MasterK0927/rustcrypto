/**
 * Cipher
 * - algorithm: The encryption algorithm to use (caesar, vigenere, xor)
 * - key: The encryption key
 */
pub struct Cipher {
    algorithm: String,
    key: String,
}

/**
 * Cipher implementation
 * - algorithm: The encryption algorithm to use (caesar, vigenere, xor)
 * - key: The encryption key
 */
impl Cipher {

    pub fn new(algorithm: String, key: String) -> Self {
        Cipher {
            algorithm,
            key,
        }
    }
    
    /**
     * encrypt
     * - message: The message to encrypt
     * - Returns: The encrypted message
     * - Uses the same algorithm as decrypt
     * - The key is case-insensitive
     * - Non-alphabetic characters are not encrypted
     * - The key is repeated to match the length of the message
     */
    pub fn encrypt(&self, message: &str) -> String {
        match self.algorithm.as_str() {
            "caesar" => self.caesar_cipher(message, true),
            "vigenere" => self.vigenere_cipher(message, true),
            "xor" => self.xor_cipher(message),
            _ => panic!("Unsupported algorithm: {}", self.algorithm),
        }
    }
    
    /**
     * decrypt
     * - message: The message to decrypt
     * - Returns: The decrypted message
     * - Uses the same algorithm as encrypt
     * - The key is case-insensitive
     * - Non-alphabetic characters are not decrypted
     * - The key is repeated to match the length of the message
     */
    pub fn decrypt(&self, message: &str) -> String {
        match self.algorithm.as_str() {
            "caesar" => self.caesar_cipher(message, false),
            "vigenere" => self.vigenere_cipher(message, false),
            "xor" => self.xor_cipher(message),
            _ => panic!("Unsupported algorithm: {}", self.algorithm),
        }
    }
    
    /**
    * caesar_cipher
    * - message: The message to encrypt/decrypt
    * - encrypt: Whether to encrypt (true) or decrypt (false)
    * - Returns: The encrypted/decrypted message
    * - Generates a Caesar cipher using the key
    * - The key is parsed as a number
    * - Non-alphabetic characters are not encrypted 
    * - The key is case-insensitive
    */
    fn caesar_cipher(&self, message: &str, encrypt: bool) -> String {
        // Parse the key as a number
        let shift = self.key.parse::<i32>().unwrap_or(3) % 26;
        let shift = if encrypt { shift } else { 26 - shift } as u8;
        
        message.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = ((c as u8 - base + shift) % 26) + base;
                    offset as char
                } else {
                    c
                }
            })
            .collect()
    }
    
    /**
     * vigenere_cipher
     * - message: The message to encrypt/decrypt
     * - encrypt: Whether to encrypt (true) or decrypt (false)
     * - Returns: The encrypted/decrypted message
     * - Generates a Vigenère cipher using the key
     * - The key is repeated to match the length of the message
     * - Non-alphabetic characters are not encrypted
     * - The key is case-insensitive
     */
    fn vigenere_cipher(&self, message: &str, encrypt: bool) -> String {
        let key = self.key.to_lowercase();
        let key_bytes: Vec<u8> = key.bytes().filter(|&b| b.is_ascii_alphabetic()).collect();
        
        if key_bytes.is_empty() {
            return message.to_string();
        }
        
        let mut result = String::with_capacity(message.len());
        let mut key_idx = 0;
        
        for c in message.chars() {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let k = key_bytes[key_idx % key_bytes.len()] - b'a';
                
                let offset = if encrypt {
                    ((c as u8 - base + k) % 26) + base
                } else {
                    ((c as u8 - base + 26 - k) % 26) + base
                };
                
                result.push(offset as char);
                key_idx += 1;
            } else {
                result.push(c);
            }
        }
        result
    }
    
    /**
     * xor_cipher
     * - message: The message to encrypt/decrypt
     * - Returns: The encrypted/decrypted message
     * - Generates a simple XOR cipher using the key
     */
    fn xor_cipher(&self, message: &str) -> String {
        let key_bytes = self.key.as_bytes();
        
        if key_bytes.is_empty() {
            return message.to_string();
        }
        
        message.bytes()
            .enumerate()
            .map(|(i, b)| (b ^ key_bytes[i % key_bytes.len()]) as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        let cipher = Cipher::new("caesar".to_string(), "3".to_string());
        let message = "Hello, World!";
        
        let encrypted = cipher.encrypt(message);
        assert_eq!(encrypted, "Khoor, Zruog!");
        
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, message);
    }
    
    #[test]
    fn test_vigenere_cipher() {
        let cipher = Cipher::new("vigenere".to_string(), "key".to_string());
        let message = "Hello, World!";
        
        let encrypted = cipher.encrypt(message);
        assert_eq!(encrypted, "Rijvs, Uyvjn!");
        
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, message);
    }
    
    #[test]
    fn test_xor_cipher() {
        let cipher = Cipher::new("xor".to_string(), "key".to_string());
        let message = "Hello, World!";
        
        let encrypted = cipher.encrypt(message);
        let decrypted = cipher.decrypt(&encrypted);
        
        assert_eq!(decrypted, message);
    }
}