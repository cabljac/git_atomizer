use clap::Parser;
use gix::{self, refs::FullName};

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

    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(e) => {
            println!("Error: Not in a git repository - {}", e);
            return;
        }
    };

    // We should probably match? or

    // So a Result is Ok or Err
    let head_name = match repo.head_name() {
        Ok(name) => name,
        Err(e) => {
            println!("Error: could not read HEAD, is repository corrupted? {}", e);
            return;
        }
    };

    if let Some(head_name) = head_name {
        let head_str = head_name.to_string();

        let branch_name = head_str.strip_prefix("refs/heads/").unwrap_or(&head_str);
        println!("On branch {}", branch_name);
    }
}
