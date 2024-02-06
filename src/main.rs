use encryptions::Encryption;

use crate::commands::{Commands, CommandsConfig};
use std::process::exit;

mod commands;
mod encryptions;

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
    
    let encryption = Encryption::new(key, iv);

    match command_config.command {
        Commands::Encrypt => {
            let encryption = encryption.handle_encrypt(&command_config.option);
            match encryption {
                Ok(_) => println!("Successfully encrypted file {}", command_config.option),
                Err(e) => println!("Something went wrong {}", e),
            }
        }
        Commands::Decrypt => {
            let decryption = encryption.handle_decrypt(&command_config.option);
            match decryption {
                Ok(_) => println!("Successfully decrypted file {}", command_config.option),
                Err(e) => println!("Something went wrong {}", e),
            }
        }
        _ => println!("Unregnized command"),
    }
}
