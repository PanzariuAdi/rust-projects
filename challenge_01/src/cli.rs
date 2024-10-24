use clap::Parser;

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
