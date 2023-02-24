/**
 *  password-based encryption, then you should use a key derivation function, such as PBKDF2. A key derivation function
 *  takes a salt and a supplied user password, and produces a key that can be used with a cipher like AES.
 *
 *  To encrypt, you would prompt for a password, generate a random salt, and derive a key using the KDF.
 *  You would then use that key with AES in a suitable block cipher mode to encrypt the data, and store only the salt and the encrypted
 *  data (and whatever IV the cipher mode requires).
 *
 *  To decrypt, you would prompt for a password, load the salt from the file, and re-derive the key. You would then use that key
 *  to decrypt the file.
 *
 *  The purpose of the salt is to prevent precomputation optimisations from being applied to a dictionary attack.
 *  It is indeed possible to perform a bruteforce dictionary attack once the salt is known,
 *  but the KDF is designed to be slow enough to make this infeasible without precomputation.
 *
 * The encryption algorithm used is AES-256-GCM.
 */
use openssl::{
    pkcs5::pbkdf2_hmac,
    rand::rand_bytes,
    symm::{decrypt_aead, encrypt_aead, Cipher},
};
use std::env;

use crate::errors::Error;

fn get_key(salt: &[u8], key: &mut [u8]) -> Result<(), Error> {
    let pass = match env::var("ENCRYPTION_SECRET") {
        Ok(var) => var,
        _ => panic!("No ENCRYPTION_SECRET value in env"),
    };

    pbkdf2_hmac(
        pass.as_bytes(),
        salt,
        10000,
        openssl::hash::MessageDigest::sha512(),
        key,
    )?;

    Ok(())
}

/**
 * Decode base64  strings.
 */
fn decode(text: &str) -> Result<Vec<u8>, Error> {
    match openssl::base64::decode_block(text) {
        Ok(val) => Ok(val),
        Err(err) => Err(Error::Openssl(err)),
    }
}

fn encrypt(text: &[u8]) -> Result<String, Error> {
    let cipher = Cipher::aes_256_gcm();

    let mut tag = vec![0; 16];
    let mut iv = vec![0; 16];
    rand_bytes(&mut iv)?;
    let mut salt = vec![0; 64];
    rand_bytes(&mut salt)?;
    let mut key = [0; 32];
    get_key(&salt, &mut key)?;

    let encrypted = encrypt_aead(cipher, &key, Some(&iv), &[], text, &mut tag)?;

    Ok(openssl::base64::encode_block(
        &[salt, iv, tag, encrypted].concat(),
    ))
}

pub fn encrypt_data(data: String) -> Result<String, Error> {
    match env::var("ENCRYPTION_SECRET") {
        Ok(..) => encrypt(data.as_bytes()),
        _ => Err(Error::Conflict(
            "failed to find ENCRYPTION_SECRET in env".to_owned(),
        )),
    }
}

fn decrypt(text: String) -> Result<String, Error> {
    let ciphertext = decode(&text)?;
    let cipher = Cipher::aes_256_gcm();

    let iv_length = 16;
    let salt_length = 64;
    let tag_length = 16;
    let tag_position = salt_length + iv_length;
    let encrypted_position = tag_position + tag_length;

    let salt: &[u8] = &ciphertext[0..salt_length];
    let iv: &[u8] = &ciphertext[salt_length..tag_position];
    let tag: &[u8] = &ciphertext[tag_position..encrypted_position];
    let encrypted: &[u8] = &ciphertext[encrypted_position..];

    let mut key = [0; 32];
    get_key(salt, &mut key)?;

    let value = decrypt_aead(cipher, &key, Some(iv), &[], encrypted, tag)?;

    Ok(String::from_utf8_lossy(&value).to_string())
}

pub fn decrypt_data(value: String) -> Result<String, Error> {
    match env::var("ENCRYPTION_SECRET") {
        Ok(..) => {
            let value = decrypt(value)?;
            Ok(value)
        }
        _ => {
            // send error if ENCRYPTION_SECRET is not set
            let value = value;
            Ok(value)
        }
    }
}
