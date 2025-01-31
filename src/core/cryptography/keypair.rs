use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use std::{fs, sync::Arc};

use crate::core::configs::app_config::APP_CONFIG;
use crate::core::cryptography::algorithms::elliptic::{generate_elliptic_keypair, CurveAlgorithms};
use crate::core::cryptography::algorithms::rsa::generate_rsa_keypair;

/// File names for the key pairs
pub const ELLIPTIC_KEY: &str = "elliptic_key.pem";
pub const RSA_KEY: &str = "rsa_key.pem";

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
    // Generate RSA key pair
    generate_rsa_keypair(
        4096,
        "keys/private/access_private_rsa_key.pem",
        "keys/public/access_public_rsa_key.pem",
    )?;

    generate_rsa_keypair(
        4096,
        "keys/private/refresh_private_rsa_key.pem",
        "keys/public/refresh_public_rsa_key.pem",
    )?;

    // Generate Elliptic key pair
    generate_elliptic_keypair(
        CurveAlgorithms::SECP256K1,
        "keys/private/access_private_elliptic_key.pem",
        "keys/public/access_public_elliptic_key.pem",
    )?;

    generate_elliptic_keypair(
        CurveAlgorithms::SECP256K1,
        "keys/private/refresh_private_elliptic_key.pem",
        "keys/public/refresh_public_elliptic_key.pem",
    )?;

    Ok(())
}

/// A key pair for signing and verifying JWTs.
/// The key pair includes a private key and a public key.
/// With the private key, we can sign a JWT
/// and with the public key, we can verify a JWT.
pub struct KeyPair {
    pub access_private_key: EncodingKey,
    pub access_public_key: DecodingKey,
    pub refresh_private_key: EncodingKey,
    pub refresh_public_key: DecodingKey,
}

/// Reads the key pair from the private and public keys in the `private` directory.
/// Firstly, Retried a root directory of the workspace.
/// Then, reads the private key from the `private` directory.
/// Finally, reads the public key from the `public` directory.
impl KeyPair {
    pub fn read_from_file(keypair_type: KeyPairType) -> Self {
        // Read Access
        let access_private_key_path = match keypair_type {
            KeyPairType::Elliptic => format!("keys/private/access_private_{}", ELLIPTIC_KEY),
            KeyPairType::RSA => format!("keys/private/access_private_{}", RSA_KEY),
        };

        let access_public_key_path = match keypair_type {
            KeyPairType::Elliptic => format!("keys/public/access_public_{}", ELLIPTIC_KEY),
            KeyPairType::RSA => format!("keys/public/access_public_{}", RSA_KEY),
        };

        // Read Refresh
        let refresh_private_key_path = match keypair_type {
            KeyPairType::Elliptic => format!("keys/private/refresh_private_{}", ELLIPTIC_KEY),
            KeyPairType::RSA => format!("keys/private/refresh_private_{}", RSA_KEY),
        };

        let refresh_public_key_path = match keypair_type {
            KeyPairType::Elliptic => format!("keys/public/refresh_public_{}", ELLIPTIC_KEY),
            KeyPairType::RSA => format!("keys/public/refresh_public_{}", RSA_KEY),
        };

        let access_private_key =
            fs::read_to_string(access_private_key_path).expect("Failed to read private key");

        let access_public_key =
            fs::read_to_string(access_public_key_path).expect("Failed to read public key");

        let refresh_private_key =
            fs::read_to_string(refresh_private_key_path).expect("Failed to read private key");
        let refresh_public_key =
            fs::read_to_string(refresh_public_key_path).expect("Failed to read public key");

        match keypair_type {
            KeyPairType::Elliptic => KeyPair {
                access_private_key: EncodingKey::from_ec_pem(access_private_key.as_bytes())
                    .unwrap(),
                access_public_key: DecodingKey::from_ec_pem(access_public_key.as_bytes()).unwrap(),
                refresh_private_key: EncodingKey::from_ec_pem(refresh_private_key.as_bytes())
                    .unwrap(),
                refresh_public_key: DecodingKey::from_ec_pem(refresh_public_key.as_bytes())
                    .unwrap(),
            },
            KeyPairType::RSA => KeyPair {
                access_private_key: EncodingKey::from_rsa_pem(access_private_key.as_bytes())
                    .unwrap(),
                access_public_key: DecodingKey::from_rsa_pem(access_public_key.as_bytes()).unwrap(),
                refresh_private_key: EncodingKey::from_rsa_pem(refresh_private_key.as_bytes())
                    .unwrap(),
                refresh_public_key: DecodingKey::from_rsa_pem(refresh_public_key.as_bytes())
                    .unwrap(),
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
