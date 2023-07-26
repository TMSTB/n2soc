use clap::Parser;

mod combine;

/// Nioh 2 save data ownership converter.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of save data of your user
    #[arg(short, long, required = true)]
    user_data: String,

    /// Path of save data of game progress
    #[arg(short, long, required = true)]
    progress_data: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = combine::combine(
        args.user_data.as_str(),
        args.progress_data.as_str()) {
        eprintln!("Error: {}", e);
    }
}
