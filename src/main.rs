use clap::Parser;
use gix::{self};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    branch: Option<String>,
}

fn print_branch_commit(
    repo: &gix::Repository,
    ref_name: &str,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Box is smart pointer, enables dynamic sizing because only need to know pointer at compile time

    let mut reference = repo.find_reference(ref_name)?;
    let commit = reference.peel_to_commit()?;
    let message = commit.message()?;

    println!("Latest commit on on {}: {}", label, message.summary());
    Ok(())
}

fn main() {
    let args = Cli::parse();

    let branch = if let Some(branch_name) = args.branch {
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
    // So a Result is Ok or Err
    let head_name = match repo.head_name() {
        Ok(name) => name,
        Err(e) => {
            println!("Error: could not read HEAD, is repository corrupted? {}", e);
            return;
        }
    };

    match print_branch_commit(&repo, &branch, "specified branch") {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }

    if let Some(head_name) = head_name {
        let head_str = head_name.to_string();

        match print_branch_commit(&repo, &head_str, "current HEAD") {
            Ok(_) => {}
            Err(e) => println!("Error: {}", e),
        }
    }
}
