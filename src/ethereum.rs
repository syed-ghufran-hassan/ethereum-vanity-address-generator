use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Digest, Keccak256};
use hex::encode;

/// Function to generate a random private key and return it
fn generate_private_key() -> SecretKey {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    SecretKey::new(&mut rng)
}

/// Function to generate the corresponding Ethereum address from a private key
fn generate_ethereum_address(private_key: &SecretKey) -> String {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    // Serialize the public key (remove the first byte to match Ethereum's address format)
    let public_key_serialized = public_key.serialize_uncompressed();
    let public_key_without_prefix = &public_key_serialized[1..];

    // Apply Keccak-256 to the public key to generate the Ethereum address
    let mut hasher = Keccak256::new();
    hasher.update(public_key_without_prefix);
    let result = hasher.finalize();

    // Ethereum address is the last 20 bytes of the hash
    let ethereum_address = &result[12..];

    // Return as a hex string with the '0x' prefix
    format!("0x{}", encode(ethereum_address))
}

/// Function to check if the generated address matches the desired prefix
fn matches_vanity_pattern(address: &str, pattern: &str) -> bool {
    address.starts_with(pattern)
}
