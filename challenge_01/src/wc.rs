use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct ResultWc {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    file_name: String,
}

#[derive(Parser)]
pub struct Cli {
    #[clap(short = 'l')]
    pub count_lines: bool,

    #[clap(short = 'w')]
    pub count_words: bool,

    #[clap(short = 'c')]
    pub count_chars: bool,

    #[clap(short = 'b')]
    pub count_bytes: bool,

    #[clap(short = 'm')]
    pub count_all: bool,

    pub path: std::path::PathBuf,
}

fn open_file(path: &str) -> Result<BufReader<File>, String> {
    let file = File::open(path).map_err(|_| "Error at opening the file".to_string())?;

    Ok(BufReader::new(file))
}

pub fn get_statistics() -> ResultWc {
    let args = Cli::parse();
    let file_name = args
        .path
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|file_name| file_name.to_string())
        .expect("failed to convert file name to string");

    let reader = open_file(&args.path.to_str().unwrap()).unwrap();

    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;
    let mut bytes = 0;

    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                if args.count_lines || args.count_all {
                    lines += 1;
                }
                if args.count_words || args.count_all {
                    words += line_str.split_whitespace().count();
                }
                if args.count_chars || args.count_all {
                    chars += line_str.chars().count();
                }
                if args.count_bytes {
                    bytes += line_str.len();
                }
            }
            Err(_) => {
                eprint!("error at reading line");
                break;
            }
        }
    }

    ResultWc {
        lines,
        words,
        chars,
        bytes,
        file_name,
    }
}

impl ResultWc {
    pub fn format(&self) -> String {
        let mut result = String::new();
        if self.lines > 0 {
            result.push_str(&format!("{} ", self.lines));
        }
        if self.words > 0 {
            result.push_str(&format!("{} ", self.words));
        }
        if self.chars > 0 {
            result.push_str(&format!("{} ", self.chars));
        }
        if self.bytes > 0 {
            result.push_str(&format!("{} ", self.bytes));
        }
        result.push_str(&format!("{}", self.file_name));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_open_file_success() {
        let valid_test_file = "test_file.txt";
        let mut file = File::create(valid_test_file).expect("failed to create test file");
        writeln!(file, "test file").expect("failed to write to test file");

        let result = open_file(valid_test_file);
        assert!(result.is_ok(), "expected ok, got err");

        fs::remove_file(valid_test_file).expect("error when removing test file");
    }

    #[test]
    fn test_open_file_not_found() {
        let test_file = "invalid.txt";

        let result = open_file(test_file);

        assert!(result.is_err(), "expected err, got ok");
        assert_eq!(result.unwrap_err(), "Error at opening the file");
    }
}
