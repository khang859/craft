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

                        let decrypted_file_name = option.clone();
                        let decrypted_file_name: Vec<&str> =
                            decrypted_file_name.split("/").collect();
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
    use tempfile::tempdir;

    fn setup() -> Result<(tempfile::TempDir, Encryption<'static>), Error> {
        let dir = tempdir();

        match dir {
            Ok(temp_dir) => {
                let key = b"lkwiekfgmjwkdjalwprktlwudkskdmkw";
                let iv = b"koskemwldkowmekw";
                let encryption = Encryption::new(&key, &iv);
                return Ok((temp_dir, encryption));
            }
            Err(create_temp_dir_failed) => {
                assert!(false);
                return Err(anyhow!(
                    "Unable to create temp directory {}",
                    create_temp_dir_failed
                ));
            }
        }
    }

    #[test]
    fn encrypt_decrypt() {
        let setup_result = setup();

        if let Ok((dir, encryption)) = setup_result {
            let file_path = dir.path().join("test.txt");
            let content = "Hello, i'm a test";
            let file = File::create(&file_path);

            if let Ok(mut created_file) = file {
                let _ = writeln!(created_file, "{}", content);
            } else {
                assert!(false);
            }

            let file_path_str_option = file_path.to_str();

            if let Some(file_path_str) = file_path_str_option {
                let file_path_str = file_path_str.to_string();
                let _ = encryption.handle_encrypt(&file_path_str);
                let _ = encryption.handle_decrypt(&format!("{}.enc", file_path_str));

                let decrypted_file_result = File::open(file_path);
                if let Ok(mut decrypted_file) = decrypted_file_result {
                    let mut decrypted_content = String::new();
                    let _ = decrypted_file.read_to_string(&mut decrypted_content);
                    assert_eq!(decrypted_content.trim_end(), content);
                }
            }
        }
    }

    #[test]
    fn file_dont_exist() {
        let (_, encryption) = setup().unwrap();
        let file_path = "fake_file_path.txt";
        let result = encryption.handle_decrypt(&file_path.to_string());
        assert!(result.is_err());

        let result = encryption.handle_encrypt(&file_path.to_string());
        assert!(result.is_err());
    }
}
