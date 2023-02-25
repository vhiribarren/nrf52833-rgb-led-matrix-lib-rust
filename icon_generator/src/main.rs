use clap::Parser;

#[derive(Parser)]
struct Cli {
    in_image: std::path::PathBuf,
    out_rust: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
}
