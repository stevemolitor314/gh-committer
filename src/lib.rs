use reqwest::{
    blocking::Client,
    header::{HeaderValue, ACCEPT, CONTENT_LENGTH, USER_AGENT},
};

const REPOS_URL: &str = "https://api.github.com/repos";

fn get_file_url(owner: &str, repo: &str, file_path: &str) -> String {
    format!("{}/{}/{}/contents/{}", REPOS_URL, owner, repo, file_path)
}

/// Get a file from a GitHub repo as a string
// TODO convert reqwest::Error to a custom error
pub fn get_file(
    owner: &str,
    repo: &str,
    branch: &str,
    file_path: &str,
) -> Result<String, reqwest::Error> {
    let url = get_file_url(owner, repo, file_path);
    let client = Client::new();

    client
        .get(url)
        .header(ACCEPT, "application/vnd.github.v3.raw")
        .header(CONTENT_LENGTH, HeaderValue::from_static("0"))
        .header(USER_AGENT, "gh-committer")
        .query(&[("ref", branch)])
        .send()?
        .text()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_url() {
        let url = get_file_url("owner", "repo", "src/file.txt");
        assert_eq!(
            "https://api.github.com/repos/owner/repo/contents/src/file.txt",
            url
        );
    }
}
