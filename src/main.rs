use crate::commands::{Commands, CommandsConfig};
use anyhow::{anyhow, Error};
use openssl::symm::{Cipher, Crypter, Mode};
use std::fs::{self, File};
use std::io::Write;
use std::process::exit;

mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command_config = CommandsConfig::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    // Todo: Add adctual password input process; 
    let key = b"0123456789abcdef0123456789abcdef";

    // Todo: Save IV somewhere else;
    let iv = b"1234567890abcdef";

    match command_config.command {
        Commands::Encrypt => {
            let encryption = handle_encrypt(&command_config.option, key, iv);
            match encryption {
                Ok(_) => println!("Successfully encrypted file {}", command_config.option),
                Err(e) => println!("Something went wrong {}", e),
            }
        }
        Commands::Decrypt => println!("decrypt"),
    }
}

fn handle_encrypt(option: &String, key: &[u8], iv: &[u8]) -> Result<(), Error> {
    match fs::metadata(option) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(anyhow!("Target needs to be a file"));
            }

            let file_content = fs::read(option);

            match file_content {
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
