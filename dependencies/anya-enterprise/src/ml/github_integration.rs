use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use mockall::{automock, predicate::*};

#[derive(Serialize)]
struct Issue {
	title: String,
	body: String,
}

#[cfg(test)]
mod tests {
	use super::*;
	use tokio::runtime::Runtime;

	#[test]
	fn test_create_issue() {
		let rt = Runtime::new().unwrap();
		let github_integrator = GitHubIntegrator::new("fake_token".to_string());
		let issue = Issue {
			title: "Test Issue".to_string(),
			body: "This is a test issue.".to_string(),
		};
		let result = rt.block_on(github_integrator.create_issue("fake_repo/fake_project", issue));
		assert!(result.is_err()); // Expecting an error due to fake token and repo
	}
}

pub struct GitHubIntegrator {
	client: Client,
	token: String,
}
#[automock]
impl GitHubIntegrator {
impl GitHubIntegrator {
	pub fn new(token: String) -> Self {
		GitHubIntegrator {
			client: Client::new(),
			token,
		}
	}

	pub async fn create_issue(&self, repo: &str, issue: Issue) -> Result<(), Box<dyn Error>> {
		let url = format!("https://api.github.com/repos/{}/issues", repo);
		let response = self.client.post(&url)
			.bearer_auth(&self.token)
			.json(&issue)
			.send()
			.await?;
		if response.status().is_success() {
			println!("Issue created successfully");
		} else {
			println!("Failed to create issue: {:?}", response.text().await?);
		}
		Ok(())
	}
}