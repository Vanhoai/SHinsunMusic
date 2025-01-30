use openssl::{pkey::PKey, rsa::Rsa};
use std::{fs::File, io::Write, path::Path};

/// Generates a RSA keypair with the given key size and saves it to the given paths.
pub fn generate_rsa_keypair(
    key_size: u32,
    private_key_path: &str,
    public_key_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(private_key_path).exists() && Path::new(public_key_path).exists() {
        return Ok(());
    }

    let rsa = Rsa::generate(key_size)?;
    let keypair = PKey::from_rsa(rsa)?;

    let private_key_pem = keypair.private_key_to_pem_pkcs8()?;

    println!("Create private key file: {}", private_key_path);
    let mut private_key_file = File::create(private_key_path)?;
    println!("Write private key to file");
    private_key_file.write_all(&private_key_pem)?;

    println!("Serializing public key to PEM format");
    let public_key_pem = keypair.public_key_to_pem()?;
    let mut public_key_file = File::create(public_key_path)?;
    public_key_file.write_all(&public_key_pem)?;

    Ok(())
}
