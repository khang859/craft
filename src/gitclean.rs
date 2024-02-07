use std::{fs::File, io::Read};

use anyhow::Error;

pub fn handle_gitclean(path: String) -> Result<(), Error> {
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let _: Vec<&str> = content
        .lines()
        .map(|line| {
            println!("{:?}", line);
            return line;
        })
        .collect();

    return Ok(());
}
