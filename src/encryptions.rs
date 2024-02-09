use anyhow::{anyhow, Error};
use openssl::symm::{Cipher, Crypter, Mode};
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug)]
pub struct Encryption<'a> {
    pub key: &'a [u8; 32],
    pub iv: &'a [u8; 16],
}

impl<'a> Encryption<'a> {
    pub fn new(key: &'a [u8; 32], iv: &'a [u8; 16]) -> Self {
        return Encryption { key, iv };
    }

    pub fn handle_decrypt(&self, option: &String) -> Result<(), Error> {
        match fs::metadata(option) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err(anyhow!("Target needs to be a file"));
                }

                match fs::read(option) {
                    Ok(content) => {
                        let mut decrypter = Crypter::new(
                            Cipher::aes_256_cbc(),
                            Mode::Decrypt,
                            self.key,
                            Some(self.iv),
                        )?;

                        let block_size = Cipher::aes_256_cbc().block_size();
                        let mut decrypted_data = vec![0; content.len() + block_size];
                        let count = decrypter.update(&content, &mut decrypted_data)?;
                        let rest = decrypter.finalize(&mut decrypted_data[count..])?;
                        decrypted_data.truncate(count + rest);

                        let decrypted_file_path = option.clone();
                        let decrypted_file_path: Vec<&str> =
                            decrypted_file_path.split("/").collect();

                        let decrypted_file_name =
                            decrypted_file_path.get(decrypted_file_path.len() - 1);

                        // Obtain everything but the last item.
                        let decrypted_file_dir =
                            decrypted_file_path.get(0..decrypted_file_path.len() - 2);

                        match (decrypted_file_name, decrypted_file_dir) {
                            (Some(file_name), Some(file_dir)) => {
                                let file_name: Vec<&str> = file_name.split(".").collect();
                                let file_name = file_name.get(0..2);

                                if let Some(file_strs) = file_name {
                                    let finalized_file_name =
                                        format!("{}/{}", file_dir.join("/"), file_strs.join("."));

                                    let mut output_file =
                                        File::create(finalized_file_name.clone())?;
                                    output_file.write_all(&decrypted_data)?;
                                    return Ok(());
                                } else {
                                    Err(anyhow!("Unabled to parse file name"))
                                }
                            }
                            _ => Err(anyhow!("Couldn't parse file name and dir")),
                        }
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

    pub fn handle_encrypt(&self, option: &String) -> Result<(), Error> {
        match fs::metadata(option) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err(anyhow!("Target needs to be a file"));
                }

                match fs::read(option) {
                    Ok(content) => {
                        let mut encrypter = Crypter::new(
                            Cipher::aes_256_cbc(),
                            Mode::Encrypt,
                            self.key,
                            Some(self.iv),
                        )?;

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
}

#[cfg(test)]
mod encryption_tests {
    use std::io::Read;

    use super::*;
    use tempfile::NamedTempFile;

    fn setup() -> (tempfile::NamedTempFile, Encryption<'static>) {
        let temp_file = NamedTempFile::new().expect("Failed to create tempfile");

        let key = b"lkwiekfgmjwkdjalwprktlwudkskdmkw";
        let iv = b"koskemwldkowmekw";
        let encryption = Encryption::new(&key, &iv);
        return (temp_file, encryption);
    }

    #[test]
    fn encrypt_decrypt() {
        let (mut temp_file, encryption) = setup();

        let content = "Hello, i'm a test";

        let _ = writeln!(temp_file, "{}", content);
        let temp_file_path = temp_file
            .path()
            .to_str()
            .expect("Failed to convert file path to string");

        let _ = encryption.handle_encrypt(&String::from(temp_file_path));
        let _ = encryption.handle_decrypt(&format!("{}.enc", temp_file_path));

        let mut decrypted_file_result = File::open(&temp_file).expect("Failed to open temp file");
        let mut decrypted_content = String::new();
        let _ = decrypted_file_result.read_to_string(&mut decrypted_content);

        assert_eq!(decrypted_content.trim_end(), content);
    }

    #[test]
    fn file_dont_exist() {
        let (_, encryption) = setup();
        let file_path = "fake_file_path.txt";
        let result = encryption.handle_decrypt(&file_path.to_string());
        assert!(result.is_err());

        let result = encryption.handle_encrypt(&file_path.to_string());
        assert!(result.is_err());
    }
}
