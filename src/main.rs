// anyhow Error wraps any Error
use anyhow::{Context, Result};
use clap::Parser;
use gix::{self};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    branch: Option<String>,
}

// I wanted to factor this out, but:
// 1. is it even worth doing? we could add common with_context I guess, just thinking DRY
// 2. I was having issues with lifecycle ' annotations, they're something i haven't learnt much of yet
fn _get_reference<'a>(repo: &'a gix::Repository, ref_name: &str) -> Result<gix::Reference<'a>> {
    repo.find_reference(ref_name)
        .context("Failed to find reference")
}

// 1.

// This would be dangerous!
// let reference = {
//     let repo = gix::discover(".")?;
//     get_reference(&repo, "main")?
// };  // repo dropped here!
// reference points to freed memory - Rust says NO!

// 2. Not really, since it's a tiny bit of code. better to extract get_commit_from_reference or something

fn print_branch_commit(
    repo: &gix::Repository,
    ref_name: &str,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Box is smart pointer, enables dynamic sizing because only need to know pointer at compile time

    let mut reference = repo
        .find_reference(ref_name)
        .with_context(|| format!("Cannot find reference for {}", label))?;

    // anyhow context or with_context converts to Result

    let commit = reference
        .peel_to_commit()
        .context("Failed to peel reference from commit")?;

    let message = commit.message().context("Failed to read commit message")?;

    println!("Latest commit on on {}: {}", label, message.summary());
    Ok(())
}

// Add this function after print_branch_commit
fn explore_commit_tree(repo: &gix::Repository, ref_name: &str) -> Result<()> {
    // Note anyhow doesn't need to provide the Box pointer type annotation, i think it comes with it.

    // TODO: Your task is to:
    // 1. Get the reference (similar to print_branch_commit)

    let mut reference = repo
        .find_reference(ref_name)
        .context("Failed to get reference")?;

    let commit = reference
        .peel_to_commit()
        .context("Failed to peel to commit")?;

    let tree = commit.tree().context("Failed to get tree from commit")?;

    //  this errors with "no method named `context` found for struct `gix::Id<'_>` in the current scope"

    // I suppose id is not a Result<> type? So cannot be matched/unwrapped
    // let id = tree.id().context(|| "Cound not get tree ID")?;

    let id = tree.id();

    println!("Tree ID: {}", id);

    // Or like this? or is Object ID different or something:

    // Answer:
    // tree.id would access a public field directly
    // tree.id() calls a method
    // The Id type implements Display, so can use println! macro.

    // println!("Tree ID: {}", tree.id);

    // 2. Peel to commit
    // 3. Get the tree from the commit using commit.tree()?
    // 4. Print the tree id using tree.id
    // 5. Iterate through tree entries and print their names
    //    Hint: use tree.iter() and for entry in ... pattern
    //    Each entry has a filename() method that returns a BString

    Ok(())
}

fn get_commit_from_ref<'a>(repo: &'a gix::Repository, ref1: &'a str) -> Result<gix::Commit<'a>> {
    let mut reference = repo.find_reference(ref1).context("...")?;
    reference
        .peel_to_commit()
        .context("Failed to peel reference from commit")
}

fn _get_commit_from_ref_2<'a>(repo: &'a gix::Repository, ref1: &'a str) -> Result<gix::Commit<'a>> {
    let mut reference = repo.find_reference(ref1).context("...")?;
    Ok(reference
        .peel_to_commit()
        .context("Failed to peel reference from commit")?)
}

fn compare_commits(repo: &gix::Repository, ref1: &str, ref2: &str) -> Result<()> {
    let commit1 = get_commit_from_ref(repo, ref1)
        .with_context(|| format!("Failed to get commit {}", ref1))?;
    let commit2 = get_commit_from_ref(repo, ref2)
        .with_context(|| format!("Failed to get commit {}", ref2))?;

    println!("Comparing commits {}, {}", commit1.id(), commit2.id());

    Ok(())

    // gix::diff::tree_with_rewrites
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
    // let head_name = match repo.head_name() {
    //     Ok(name) => name,
    //     Err(e) => {
    //         println!("Error: could not read HEAD, is repository corrupted? {}", e);
    //         return;
    //     }
    // };

    match print_branch_commit(&repo, &branch, "specified branch") {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }

    match explore_commit_tree(&repo, &branch) {
        Ok(_) => {}
        Err(e) => println!("Error exploring tree: {}", e),
    }

    match compare_commits(&repo, &branch, &branch) {
        Ok(_) => {}
        Err(e) => println!("Error comparing commits {}", e),
    }

    // if let Some(head_name) = head_name {
    //     let head_str = head_name.to_string();

    //     match print_branch_commit(&repo, &head_str, "current HEAD") {
    //         Ok(_) => {}
    //         Err(e) => println!("Error: {}", e),
    //     }
    // }
}
