use std::{fs::File, io::Read};

use anyhow::Error;

enum GitType {
    Clean,
    Checkout,
}
pub struct GitPath {
    git_type: GitType,
    path: String,
}

pub fn handle_gitclean(path: String) -> Result<(), Error> {
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let _: Vec<GitPath> = content
        .lines()
        .map(|line| {
            let _ = parse_line(line);
            return GitPath {
                git_type: GitType::Checkout,
                path: String::from("/path/file.txt"),
            };
        })
        .collect();

    return Ok(());
}

fn parse_line(line: &str) -> Result<String, Error> {
    println!("{:?}", line);
    let line_parse_result: Vec<&str> = line.split(":").collect();

    println!("{:?}", line_parse_result);

    return Ok(String::from("test"));
}

#[cfg(test)]
mod gitclean_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn setup() -> tempfile::NamedTempFile {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        return temp_file;
    }

    #[test]
    fn test_handle_gitclean() -> Result<(), Error> {
        let mut file = setup();
        let file_content = "modified: /test/file.txt\ndeleted: /another/test/file/this/time/is.yml\n/some/other/file/now.yml\n";
        let _ = writeln!(file, "{}", file_content);
        return Ok(());
    }
}
