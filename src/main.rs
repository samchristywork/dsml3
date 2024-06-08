use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to read
    #[arg(short, long)]
    input: String,

    /// Name of the output file
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
}
