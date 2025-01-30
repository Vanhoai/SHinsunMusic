use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use std::path::Path;
use std::{fs, sync::Arc};

use crate::core::configs::app_config::APP_CONFIG;
use crate::core::cryptography::algorithms::elliptic::{generate_elliptic_keypair, CurveAlgorithms};
use crate::core::cryptography::algorithms::rsa::generate_rsa_keypair;

/// File names for the key pairs
pub const PRIVATE_ELLIPTIC_KEY: &str = "private_elliptic_key.pem";
pub const PUBLIC_ELLIPTIC_KEY: &str = "public_elliptic_key.pem";
pub const PRIVATE_RSA_KEY: &str = "private_rsa_key.pem";
pub const PUBLIC_RSA_KEY: &str = "public_rsa_key.pem";

/// The type of the key pair.
pub enum KeyPairType {
    Elliptic,
    RSA,
}

impl KeyPairType {
    pub fn from(key_type: &str) -> Self {
        match key_type {
            "Elliptic" => KeyPairType::Elliptic,
            "RSA" => KeyPairType::RSA,
            _ => panic!("Invalid key type"),
        }
    }
}

pub fn generate_key_pair() -> Result<(), Box<dyn std::error::Error>> {
    generate_rsa_keypair(
        4096,
        "private/private_rsa_key.pem",
        "private/public_rsa_key.pem",
    )?;

    generate_elliptic_keypair(
        CurveAlgorithms::SECP256K1,
        "private/private_elliptic_key.pem",
        "private/public_elliptic_key.pem",
    )?;

    Ok(())
}

/// A key pair for signing and verifying JWTs.
/// The key pair includes a private key and a public key.
/// With the private key, we can sign a JWT
/// and with the public key, we can verify a JWT.
pub struct KeyPair {
    pub private_key: EncodingKey,
    pub public_key: DecodingKey,
}

/// Reads the key pair from the private and public keys in the `private` directory.
/// Firstly, Retried a root directory of the workspace.
/// Then, reads the private key from the `private` directory.
/// Finally, reads the public key from the `public` directory.
impl KeyPair {
    pub fn read_from_file(keypair_type: KeyPairType) -> Self {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

        let private_key_path = workspace_root.join("private").join(match keypair_type {
            KeyPairType::Elliptic => PRIVATE_ELLIPTIC_KEY,
            KeyPairType::RSA => PRIVATE_RSA_KEY,
        });

        let public_key_path = workspace_root.join("private").join(match keypair_type {
            KeyPairType::Elliptic => PUBLIC_ELLIPTIC_KEY,
            KeyPairType::RSA => PUBLIC_RSA_KEY,
        });

        let private_key = fs::read_to_string(private_key_path).expect("Failed to read private key");
        let public_key = fs::read_to_string(public_key_path).expect("Failed to read public key");

        match keypair_type {
            KeyPairType::Elliptic => KeyPair {
                private_key: EncodingKey::from_ec_pem(private_key.as_bytes()).unwrap(),
                public_key: DecodingKey::from_ec_pem(public_key.as_bytes()).unwrap(),
            },
            KeyPairType::RSA => KeyPair {
                private_key: EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap(),
                public_key: DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
            },
        }
    }
}

/// A lazy-loaded key pair.
/// This is used to avoid reading the key pair from the file every time it is needed.
/// The key pair is read from the file only once when the program starts.
/// The key pair is then stored in the `KEY_PAIR` static variable.
/// Sample usage:
/// ``` Rust
/// use cryptography::keypair::KEY_PAIR;
///
/// let token = jwt::encode(&KEY_PAIR.private_key, payload, None).unwrap();
/// let payload = jwt::decode::<Payload>(&KEY_PAIR.public_key, token, None).unwrap();
/// ```
pub static KEY_PAIR: Lazy<Arc<KeyPair>> = Lazy::new(|| {
    Arc::new(KeyPair::read_from_file(KeyPairType::from(
        &APP_CONFIG.jwt.key_type,
    )))
});
