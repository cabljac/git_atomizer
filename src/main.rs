use clap::Parser;

/// test description!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    branch: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.branch {
        Some(branch_name) => println!("Looking at branch {}", branch_name),
        None => println!("No branch specified"),
    }
}
