mod github;
use crate::github::github_issue::get_github_issue;
use crate::github::github_repo::get_github_repo_id;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let owner: String = String::from("mryhryki");
    let repo: String = String::from("HOME");
    let issue: i32 = 50;

    let issue = get_github_issue(&owner, &repo, &issue).await?;
    let repo_id = get_github_repo_id(&owner, &repo).await?;
    println!("{:#?}", issue);
    println!("{:#?}", repo_id);
    Ok(())
}
