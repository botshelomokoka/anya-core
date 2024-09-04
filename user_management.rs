use std::env;
use std::collections::HashMap;
use std::error::Error;
use reqwest;
use serde_json::Value;
use log::{info, error};
use crypto::aes::{cbc_encryptor, cbc_decryptor, KeySize};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult};
use rand::Rng;
use crate::setup_project::ProjectSetup;

#[derive(Default)]
struct UserState {
    github_username: String,
    user_type: String,
    encrypted_data: HashMap<String, Vec<u8>>,
}

struct UserType;

impl UserType {
    const CREATOR: &'static str = "creator";
    const NORMAL: &'static str = "normal";
    const DEVELOPER: &'static str = "developer";
}

pub struct UserManagement {
    logger: log::Logger,
    github_token: Option<String>,
    user_state: UserState,
    cipher_key: [u8; 32],
}

impl UserManagement {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let cipher_key: [u8; 32] = rng.gen();
        
        UserManagement {
            logger: log::Logger::root(log::slog_stdlog::StdLog.fuse(), o!()),
            github_token: env::var("GITHUB_TOKEN").ok(),
            user_state: UserState::default(),
            cipher_key,
        }
    }

    pub async fn identify_user(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(github_username) = self.get_github_username().await? {
            self.user_state.github_username = github_username.clone();
            if github_username == "botshelomokoka" {
                self.user_state.user_type = UserType::CREATOR.to_string();
                info!(self.logger, "Creator identified. Setting up creator-specific configurations.");
            } else if self.is_developer(&github_username).await? {
                self.user_state.user_type = UserType::DEVELOPER.to_string();
                info!(self.logger, "Developer identified. Setting up developer environment.");
            } else {
                self.user_state.user_type = UserType::NORMAL.to_string();
                info!(self.logger, "Normal user identified.");
            }
        } else {
            error!(self.logger, "Failed to identify user.");
        }
        Ok(())
    }

    async fn get_github_username(&self) -> Result<Option<String>, Box<dyn Error>> {
        match &self.github_token {
            Some(token) => {
                let client = reqwest::Client::new();
                let response = client.get("https://api.github.com/user")
                    .header("Authorization", format!("token {}", token))
                    .header("Accept", "application/vnd.github.v3+json")
                    .send()
                    .await?
                    .json::<Value>()
                    .await?;
                Ok(response["login"].as_str().map(|s| s.to_string()))
            }
            None => {
                error!(self.logger, "GitHub token not found in environment variables.");
                Ok(None)
            }
        }
    }

    async fn is_developer(&self, github_username: &str) -> Result<bool, Box<dyn Error>> {
        let developer_organizations = vec!["anya-core-developers"];
        let developer_teams = vec!["dev-team"];

        if let Some(token) = &self.github_token {
            let client = reqwest::Client::new();
            for org in developer_organizations {
                let response = client.get(&format!("https://api.github.com/orgs/{}/members/{}", org, github_username))
                    .header("Authorization", format!("token {}", token))
                    .header("Accept", "application/vnd.github.v3+json")
                    .send()
                    .await?;
                if response.status() == 204 {
                    return Ok(true);
                }

                for team in &developer_teams {
                    let response = client.get(&format!("https://api.github.com/orgs/{}/teams/{}/memberships/{}", org, team, github_username))
                        .header("Authorization", format!("token {}", token))
                        .header("Accept", "application/vnd.github.v3+json")
                        .send()
                        .await?;
                    if response.status() == 200 {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    pub fn encrypt_user_data(&mut self, data: HashMap<String, String>) {
        for (key, value) in data {
            let encrypted_value = self.encrypt(&value);
            self.user_state.encrypted_data.insert(key, encrypted_value);
        }
    }

    pub fn decrypt_user_data(&self, key: &str) -> Option<String> {
        self.user_state.encrypted_data.get(key)
            .map(|encrypted_value| self.decrypt(encrypted_value))
    }

    fn encrypt(&self, data: &str) -> Vec<u8> {
        let mut encryptor = cbc_encryptor(
            KeySize::KeySize256,
            &self.cipher_key,
            &[0u8; 16],
            crypto::blockmodes::PkcsPadding,
        );

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = RefReadBuffer::new(data.as_bytes());
        let mut buffer = [0; 4096];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);

        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        final_result
    }

    fn decrypt(&self, encrypted_data: &[u8]) -> String {
        let mut decryptor = cbc_decryptor(
            KeySize::KeySize256,
            &self.cipher_key,
            &[0u8; 16],
            crypto::blockmodes::PkcsPadding,
        );

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = RefReadBuffer::new(encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        String::from_utf8(final_result).unwrap()
    }

    pub fn get_user_state(&self) -> HashMap<String, String> {
        let mut state = HashMap::new();
        state.insert("github_username".to_string(), self.user_state.github_username.clone());
        state.insert("user_type".to_string(), self.user_state.user_type.clone());
        state
    }

    pub async fn initialize_user(&mut self) -> Result<(), Box<dyn Error>> {
        self.identify_user().await?;
        match self.user_state.user_type.as_str() {
            UserType::CREATOR => self.setup_creator_environment(),
            UserType::DEVELOPER => self.setup_developer_environment(),
            _ => self.setup_normal_user_environment(),
        }
        self.setup_project()?;
        Ok(())
    }

    fn setup_creator_environment(&self) {
        info!(self.logger, "Setting up creator environment");
        // Implement creator-specific setup
    }

    fn setup_developer_environment(&self) {
        info!(self.logger, "Setting up developer environment");
        // Implement developer-specific setup
    }

    fn setup_normal_user_environment(&self) {
        info!(self.logger, "Setting up normal user environment");
        // Implement normal user setup
    }

    fn setup_project(&self) -> Result<(), Box<dyn Error>> {
        let project_setup = ProjectSetup::new(&self.user_state.user_type, &self.get_user_state());
        project_setup.setup()?;
        Ok(())
    }
}
