use anyhow::{anyhow, Error};
use openssl::symm::{Cipher, Crypter, Mode};
use std::fs::{self, File};
use std::io::Write;

pub fn handle_decrypt(option: &String, key: &[u8], iv: &[u8]) -> Result<(), Error> {
    match fs::metadata(option) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(anyhow!("Target needs to be a file"));
            }

            match fs::read(option) {
                Ok(content) => {
                    let mut decrypter =
                        Crypter::new(Cipher::aes_256_cbc(), Mode::Decrypt, key, Some(iv))?;

                    let block_size = Cipher::aes_256_cbc().block_size();
                    let mut decrypted_data = vec![0; content.len() + block_size];
                    let count = decrypter.update(&content, &mut decrypted_data)?;
                    let rest = decrypter.finalize(&mut decrypted_data[count..])?;
                    decrypted_data.truncate(count + rest);

                    let decrypted_file_name = option.clone();
                    let decrypted_file_name: Vec<&str> = decrypted_file_name.split("/").collect();
                    let decrypted_file_name =
                        decrypted_file_name.get(decrypted_file_name.len() - 1);

                    if let Some(file_name) = decrypted_file_name {
                        println!("{:?}", file_name);

                        let file_name: Vec<&str> = file_name.split(".").collect();
                        let file_name = file_name.get(0..2);

                        if let Some(file_strs) = file_name {
                            let finalized_file_name = file_strs.join(".");

                            let mut output_file = File::create(finalized_file_name.clone())?;
                            output_file.write_all(&decrypted_data)?;
                        }
                    }

                    return Ok(());
                }
                Err(e) => {
                    return Err(anyhow!("Couldn't read file content {}", e));
                }
            }
        }
        Err(e) => {
            return Err(anyhow!("Couldn't read file metadata {}", e));
        }
    }
}

pub fn handle_encrypt(option: &String, key: &[u8], iv: &[u8]) -> Result<(), Error> {
    match fs::metadata(option) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(anyhow!("Target needs to be a file"));
            }

            match fs::read(option) {
                Ok(content) => {
                    let mut encrypter =
                        Crypter::new(Cipher::aes_256_cbc(), Mode::Encrypt, key, Some(iv))?;

                    let block_size = Cipher::aes_256_cbc().block_size();

                    let mut encrypted_data = vec![0; content.len() + block_size];

                    let count = encrypter.update(&content, &mut encrypted_data)?;
                    let rest = encrypter.finalize(&mut encrypted_data[count..])?;

                    encrypted_data.truncate(count + rest);

                    let encrypted_file_name = option.clone() + ".enc";
                    let mut output_file = File::create(encrypted_file_name)?;
                    let _ = output_file.write(&encrypted_data)?;

                    return Ok(());
                }
                Err(e) => {
                    return Err(anyhow!("Unable to read file {}", e));
                }
            }
        }
        Err(e) => {
            return Err(anyhow!("Couldn't access file {}", e));
        }
    }
}
