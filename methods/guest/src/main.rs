use k256::ecdsa::signature::Signer;
use k256::ecdsa::{Signature, SigningKey, VerifyingKey};
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};
use sha3::Keccak256;

fn main() {
    let phone_number: String = env::read();
    let password: String = env::read();

    // Text to hash
    let text = password.clone() + &phone_number;

    // Create a SHA-256 object
    let mut hasher = Sha256::new();

    // Write input message
    hasher.update(text);

    // Read hash digest and consume hasher
    let result = hasher.finalize();

    // Read hash digest and consume h
    let signing_key_result = SigningKey::from_bytes(&result);

    // Check the result of trying to create the SigningKey
    let signing_key = match signing_key_result {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Failed to create signing key: {}", e);
            return;
        }
    };
    let verify_key = VerifyingKey::from(&signing_key);
    let compressed_public_key = verify_key.to_encoded_point(false); // true for compressed

    let mut hasher = Keccak256::new();

    // Write input data
    hasher.update(compressed_public_key);
    // Read hash digest in the form of GenericArray, which in this case is equivalent to an array of u8
    let result = hasher.finalize();

    // Convert the signing key to a byte array
    let secret_bytes = signing_key.to_bytes();

    let verify_bytes = &result[result.len() - 20..];

    // Print the raw bytes in hexadecimal format for readability
    println!("Private Address: {:?}", hex::encode(secret_bytes));
    println!("Public address: 0x{}", hex::encode(verify_bytes));

    let message = b"ECDSA message to be signed";

    let signature: Signature = signing_key.sign(message);

    if password == "" {
        panic!("Password invalid. Please try again.")
    }

    let pub_address: String = "password ".to_string() + &password;

    env::commit(&pub_address);
}
