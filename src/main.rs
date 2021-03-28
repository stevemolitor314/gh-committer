use gh_committer::get_file;

fn main() {
    let owner = "stevemolitor314";
    let repo = "gh-committer";
    let branch = "main";
    let file_path = "src/main.rs";

    let resp = get_file(owner, repo, branch, file_path);
    match resp {
        Ok(text) => println!("{}", text),
        Err(err) => eprintln!("{:#?}", err),
    }
}
