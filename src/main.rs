use gh_committer::get_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let owner = "stevemolitor314";
    let repo = "gh-committer";
    let branch = "main";
    let file_path = "src/main.rs";

    let text = get_file(owner, repo, branch, file_path)?;
    println!("{}", text);

    Ok(())
}
