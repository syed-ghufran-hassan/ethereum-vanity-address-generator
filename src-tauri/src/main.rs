use rand::rngs::OsRng;
use rand::RngCore;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use hex::encode;
use std::io::{self, Write};
use tauri::command;

// Function to generate a new private key
fn generate_private_key() -> SecretKey {
    let mut rng = OsRng;
    let mut secret_key = [0u8; 32]; // Create a 32-byte array
    rng.fill_bytes(&mut secret_key); // Fill the array with random bytes
    SecretKey::from_slice(&secret_key).expect("32 bytes, within curve order")
}

// Function to generate an Ethereum address from a private key
fn generate_ethereum_address(private_key: &SecretKey) -> String {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, private_key);
    let public_key_bytes = public_key.serialize_uncompressed();

    // Hash the public key using Keccak256 (Ethereum address is last 20 bytes of hash)
    let mut hasher = Keccak256::new();
    hasher.update(&public_key_bytes[1..]); // Skip the first byte (0x04 for uncompressed pub key)
    let result = hasher.finalize();
    let address = &result[12..]; // Ethereum address is the last 20 bytes of the Keccak256 hash
    format!("0x{}", encode(address))
}

// Function to check if an Ethereum address matches a vanity pattern
fn matches_vanity_pattern(address: &str, pattern: &str) -> bool {
    address.starts_with(pattern)
}

fn main() {
    // Generate private key and Ethereum address
    let private_key = generate_private_key();
    let ethereum_address = generate_ethereum_address(&private_key);

    // Ensure private key is 32 bytes
    let secret_bytes = private_key.as_ref(); // Convert the private key to a byte slice
    assert_eq!(secret_bytes.len(), 32); // Private key should be 32 bytes

    // Get user input for a vanity pattern
    println!("Enter the vanity pattern you want to match (e.g., 0xabc): ");
    let mut pattern = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pattern).expect("Failed to read pattern");
    let pattern = pattern.trim(); // Remove newline

    // Check if the generated Ethereum address matches the vanity pattern
    if matches_vanity_pattern(&ethereum_address, pattern) {
        println!("Vanity address found: {}", ethereum_address);
    } else {
        println!("Generated address: {}", ethereum_address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_vanity_pattern() {
        assert!(matches_vanity_pattern("0xabc123", "0xabc"));
        assert!(!matches_vanity_pattern("0xdef456", "0xabc"));
    }

    #[test]
    fn test_generate_private_key() {
        let private_key = generate_private_key();
        // Assert the private key is valid
        let secret_bytes = private_key.as_ref(); // Use as_ref() to get the byte slice
        assert_eq!(secret_bytes.len(), 32); // Private key should be 32 bytes
    }

    #[test]
    fn test_generate_ethereum_address() {
        let private_key = generate_private_key();
        let address = generate_ethereum_address(&private_key);
        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42); // Ethereum addresses should be 42 characters (0x + 40 hex chars)
    }
}
