use std::{fs::File, io::Read};

use anyhow::{Error, anyhow};

#[derive(Debug, PartialEq)]
enum GitType {
    Clean,
    Checkout,
    Unstable,
}

#[derive(Debug)]
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
            let parsed_line = parse_line(line);
            match parsed_line {
                Ok(parsed) => {
                    return parsed;
                },
                Err(e) => {
                    return GitPath {
                        git_type: GitType::Unstable,
                        path: format!("{}", e),
                    }
                }
            }
        }) 
        .collect();

    return Ok(());
}

fn parse_line(line: &str) -> Result<GitPath, Error> {
    let line_parse_result: Vec<&str> = line.split(":").collect();

    if line_parse_result.len() > 1 {
        if let Some(line_path) = line_parse_result.get(1) {
            return Ok(GitPath {
                git_type: GitType::Checkout,
                path: line_path.trim().to_string(),
            });
        }
    } else {
        if let Some(line_path) = line_parse_result.get(0) {
            return Ok(GitPath {
                git_type: GitType::Clean,
                path: line_path.trim().to_string(),
            });
        }
    }
    
    return Err(anyhow!("unabled to parse line"));
}

#[cfg(test)]
mod gitclean_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn setup() -> tempfile::NamedTempFile {
        return NamedTempFile::new().expect("Failed to create temp file");
    }

    #[test]
    fn test_handle_gitclean() -> Result<(), Error> {
        let mut file = setup();
        let file_content = "modified: /test/file.txt\ndeleted: /another/test/file/this/time/is.yml\n/some/other/file/now.yml\n";
        let _ = writeln!(file, "{}", file_content);
        return Ok(());
    }

    #[test]
    fn test_parse_line() {
        let file_path = "/test/file.txt";
        let modified_str = format!("modified: {}", &file_path);
        let parsed_item = parse_line(&modified_str).expect("unabled to parse modified line");

        assert_eq!(parsed_item.git_type, GitType::Checkout);
        assert_eq!(parsed_item.path, file_path);

        let deleted_str = format!("deleted: {}", &file_path); 
        let parsed_item = parse_line(&deleted_str).expect("unabled to parse deleted line");
        
        assert_eq!(parsed_item.git_type, GitType::Checkout);
        assert_eq!(parsed_item.path, file_path);

        let parsed_item = parse_line(&file_path).expect("unabled to parse none line");

        assert_eq!(parsed_item.git_type, GitType::Clean);
        assert_eq!(parsed_item.path, file_path);
    }
}
