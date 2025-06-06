use clap::Parser;
use gix;

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

    let _branch = if let Some(branch_name) = args.branch {
        println!("Specified branch: {}", branch_name);
        branch_name
    } else {
        println!("No branch specified, using main");
        String::from("main")
    };

    match gix::discover(".") {
        Ok(_repo) => println!("Found a git repository!"),
        Err(e) => println!("Error: Not in a git repository - {}", e),
    }
}
