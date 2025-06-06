use clap::Parser;
use gix::{self};

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

    // We should probably match? or

    // So a Result is Ok or Err
    let head_name = match repo.head_name() {
        Ok(name) => name,
        Err(e) => {
            println!("Error: could not read HEAD, is repository corrupted? {}", e);
            return;
        }
    };

    let mut reference = match repo.find_reference(&branch) {
        Ok(reference) => reference,
        Err(e) => {
            println!("Error: cannot find reference for branch, {}", e);
            return;
        }
    };

    let commit = match reference.peel_to_commit() {
        Ok(commit) => commit,
        Err(e) => {
            println!("Error: could not peel, {}", e);
            return;
        }
    };

    let message = match commit.message() {
        Ok(message) => message,
        Err(e) => {
            println!("Error: could not get message from commit {}", e);
            return;
        }
    };

    println!("Latest commit on specified branch: {}", message.summary());
    // or
    println!("Latest commit message from specified branch: {:?}", message);

    if let Some(head_name) = head_name {
        let head_str = head_name.to_string();

        let branch_name = head_str.strip_prefix("refs/heads/").unwrap_or(&head_str);
        println!("On branch {}", branch_name);

        let mut reference = match repo.find_reference(&head_name) {
            Ok(reference) => reference,
            Err(e) => {
                println!("Error: could not find reference, {}", e);
                return;
            }
        };

        let commit = match reference.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => {
                println!("Error: could not peel, {}", e);
                return;
            }
        };

        let message = match commit.message() {
            Ok(message) => message,
            Err(e) => {
                println!("Error: could not get message from commit {}", e);
                return;
            }
        };

        // Add this:
        println!("Latest commit on head: {}", message.summary());
        // or
        println!("Latest commit message from head: {:?}", message);
    }
}
