#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use crate::github_api::request_github_graphql_api;

#[derive(Deserialize, Debug)]
pub struct ResponseRoot {
    data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    issue: GitHubIssue,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Debug)]
struct Variables {
    issue_number: i32,
}

pub async fn get_github_issue( issue_number: i32, ) -> Result<(), Box<dyn std::error::Error>> {
    let query = String::from("query ($issue_number: Int!) {
      user(login: \"mryhryki\") {
        repository(name: \"HOME\"){
          issue(number: $issue_number) {
            title
            url
          }
        }
      }
    }");
    let variables = Variables { issue_number };

    let response = request_github_graphql_api(query, variables).await?;
    let data = response.text().await?;
    println!("{:#?}", data);
    // let data = response.json::<ResponseRoot>().await?;
    // Ok(data.data.user.repository.issue)
    Ok(())
}
