use anyhow::{anyhow, Error};
use std::str::FromStr;

#[derive(Debug)]
pub struct CommandsConfig {
    pub command: Commands,
    pub option: String,
}

impl CommandsConfig {
    const COMMAND_IDX: usize = 1;
    const COMMAND_OPTIONS: usize = 2;

    pub fn new(args: &[String]) -> Result<CommandsConfig, Error> {
        let raw_command = args.get(Self::COMMAND_IDX);
        let raw_option = args.get(Self::COMMAND_OPTIONS);

        if let Some(command_item) = raw_command {
            let command = Self::parse_command(command_item.clone())?;
            let mut option = String::from("");
            

            if let Some(option_item) = raw_option {
                option = option_item.clone();
            } 

            return Ok(CommandsConfig {
                command,
                option,
            });
        } else {
            return Err(anyhow!("Missing command"));
        }
    }

    fn parse_command(command_config: String) -> Result<Commands, Error> {
        let config = command_config.parse::<Commands>();

        if let Ok(command_str) = config {
            return Ok(command_str);
        } else {
            return Err(anyhow!("Invalid command input"));
        }
    }
}

#[derive(Debug)]
pub enum Commands {
    Encrypt,
    Decrypt,
}

impl FromStr for Commands {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "encrypt" => Ok(Commands::Encrypt),
            "decrypt" => Ok(Commands::Decrypt),
            _ => Err(anyhow!("Invalid command")),
        }
    }
}
