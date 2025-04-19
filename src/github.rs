use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Debug)]
pub struct RepoRequest {
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub auto_init: bool,
}

#[derive(Deserialize, Debug)]
pub struct RepoResponse {
    pub html_url: String,
}

pub async fn create_github_repo(
    token: &str,
    name: &str,
    description: &Option<String>,
    private: bool,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("github-repo-creater"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    
    let repo_request = RepoRequest {
        name: name.to_string(),
        description: description.clone(),
        private,
        auto_init: true, // This creates a README.md file
    };
    
    let response = client
        .post("https://api.github.com/user/repos")
        .headers(headers)
        .json(&repo_request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("GitHub API error: {}", error_text).into());
    }
    
    let repo_response: RepoResponse = response.json().await?;
    Ok(repo_response.html_url)
}
