use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// GitHub personal access token
    #[arg(short, long)]
    pub token: String,
}
