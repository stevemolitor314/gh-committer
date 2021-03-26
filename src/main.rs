// use reqwest::{
//     blocking::Client,
//     header::{HeaderValue, ACCEPT, CONTENT_LENGTH, USER_AGENT},
// };
// use std::collections::HashMap;

//fn main() -> Result<(), Box<dyn std::error::Error>> {
// let url = "https://api.github.com/repos/stevemolitor314/what-time/contents/src/main.rs";

// let client = Client::new();
// let resp = client
//     .get(url)
//     .header(ACCEPT, "application/vnd.github.v3.raw")
//     .header(CONTENT_LENGTH, HeaderValue::from_static("0"))
//     .header(USER_AGENT, "gh-committer")
//     .send()
//     .text();
// //.json::<HashMap<String, String>>();

// println!("{:#?}", resp);
// Ok(())
//}

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
