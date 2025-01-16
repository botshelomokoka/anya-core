use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct Paper {
	title: String,
	link: String,
}

pub struct Researcher {
	client: Client,
}

impl Researcher {
	pub fn new() -> Self {
		Researcher {
			client: Client::new(),
		}
	}

	pub async fn crawl_mdpi(&self, query: &str, num_pages: usize) -> Result<Vec<Paper>, Box<dyn Error>> {
		let mut papers = Vec::new();
		for page in 1..=num_pages {
			let url = format!("https://www.mdpi.com/search?q={}&page_no={}", query, page);
			let response = self.client.get(&url).send().await?.text().await?;
			let parsed_papers: Vec<Paper> = serde_json::from_str(&response)?;
			papers.extend(parsed_papers);
		}
		Ok(papers)
	}

	pub async fn analyze_papers(&self, papers: Vec<Paper>) -> Result<(), Box<dyn Error>> {
		for paper in papers {
			println!("Analyzing paper: {}", paper.title);
			// Implement analysis logic here
		}
		Ok(())
	}
}use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct Paper {
    title: String,
    link: String,
}

pub struct Researcher {
    client: Client,
}

impl Researcher {
    pub fn new() -> Self {
        Researcher {
            client: Client::new(),
        }
    }

    pub async fn crawl_mdpi(&self, query: &str, num_pages: usize) -> Result<Vec<Paper>, Box<dyn Error>> {
        let mut papers = Vec::new();
        for page in 1..=num_pages {
            let url = format!("https://www.mdpi.com/search?q={}&page_no={}", query, page);
            let response = self.client.get(&url).send().await?.text().await?;
            let parsed_papers: Vec<Paper> = serde_json::from_str(&response)?;
            papers.extend(parsed_papers);
        }
        Ok(papers)
    }

    pub async fn analyze_papers(&self, papers: Vec<Paper>) -> Result<(), Box<dyn Error>> {
        for paper in papers {
            println!("Analyzing paper: {}", paper.title);
            // Implement analysis logic here
        }
        Ok(())
    }
}#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_crawl_mdpi() {
        let rt = Runtime::new().unwrap();
        let researcher = Researcher::new();
        let result = rt.block_on(researcher.crawl_mdpi("cybersecurity vulnerabilities", 1));
        assert!(result.is_ok());
        let papers = result.unwrap();
        assert!(!papers.is_empty());
    }

    #[test]
    fn test_analyze_papers() {
        let rt = Runtime::new().unwrap();
        let researcher = Researcher::new();
        let papers = vec![Paper {
            title: "Test Paper".to_string(),
            link: "http://example.com".to_string(),
        }];
        let result = rt.block_on(researcher.analyze_papers(papers));
        assert!(result.is_ok());
    }
}