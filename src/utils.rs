pub fn git_ssh_to_https(url: &str) -> String {
    if url.starts_with("git@") {
        url.replace("git@", "https://")
            .replace("github.com:", "github.com/")
    } else {
        url.to_string()
    }
}
