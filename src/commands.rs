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

            return Ok(CommandsConfig { command, option });
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

#[derive(Debug, PartialEq)]
pub enum Commands {
    Encrypt,
    Decrypt,
    GitClean,
}

impl FromStr for Commands {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "encrypt" => Ok(Commands::Encrypt),
            "decrypt" => Ok(Commands::Decrypt),
            "gitclean" => Ok(Commands::GitClean),
            _ => Err(anyhow!("Invalid command")),
        }
    }
}

#[cfg(test)]
mod commands_enum_tests {
    use super::*;

    #[test]
    fn test_encrypt_parse() {
        let expected = Commands::Encrypt;
        let actual_option = String::from("encrypt").parse();

        if let Ok(actual) = actual_option {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_decrypt_parse() {
        let expected = Commands::Decrypt;
        let actual_option = String::from("decrypt").parse();

        if let Ok(actual) = actual_option {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_gitclean_parse() {
        let expected = Commands::GitClean;
        let actual_option = String::from("gitclean").parse();

        if let Ok(actual) = actual_option {
            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod commands_options_tests {
    use super::*;

    #[test]
    fn test_new_success() {
        let option = String::from("/path/to/file");
        let expected = CommandsConfig {
            command: Commands::Encrypt,
            option,
        };

        let actual_input = [
            String::from("/first/arg"),
            String::from("encrypt"),
            String::from("/path/to/file"),
        ];
        let actual_option = CommandsConfig::new(&actual_input);

        match actual_option {
            Ok(actual) => {
                assert_eq!(expected.command, actual.command);
                assert_eq!(expected.option, actual.option);
            }
            Err(_) => assert!(false),
        }
    }
}
