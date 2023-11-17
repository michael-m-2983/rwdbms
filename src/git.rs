use git2::Repository;

pub fn clone_repo(repo_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    match Repository::clone_recurse(repo_url, "repo") {
        Ok(_) => Ok (()),
        Err(e) => Err (Box::new(e))
    }
}

pub fn pull_repo(repo_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    todo!();
}
