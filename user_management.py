import os
import requests
import logging
from typing import Optional, Dict, Any
from dataclasses import dataclass, field
from cryptography.fernet import Fernet
from setup_project import ProjectSetup

@dataclass
class UserState:
    github_username: str = ""
    user_type: str = ""
    encrypted_data: Dict[str, Any] = field(default_factory=dict)

class UserType:
    CREATOR = "creator"
    NORMAL = "normal"
    DEVELOPER = "developer"

class UserManagement:
    def __init__(self):
        self.logger = logging.getLogger(__name__)
        self.github_token = os.environ.get('GITHUB_TOKEN')
        self.user_state = UserState()
        self.cipher_suite = Fernet(Fernet.generate_key())

    def identify_user(self) -> None:
        """
        Identify the user type based on their GitHub username.
        """
        github_username = self.get_github_username()
        if github_username:
            self.user_state.github_username = github_username
            if github_username == "botshelomokoka":
                self.user_state.user_type = UserType.CREATOR
                self.logger.info("Creator identified. Setting up creator-specific configurations.")
            elif self.is_developer(github_username):
                self.user_state.user_type = UserType.DEVELOPER
                self.logger.info("Developer identified. Setting up developer environment.")
            else:
                self.user_state.user_type = UserType.NORMAL
                self.logger.info("Normal user identified.")
        else:
            self.logger.error("Failed to identify user.")

    def get_github_username(self) -> Optional[str]:
        """
        Fetch the GitHub username using the provided GitHub token.
        """
        if not self.github_token:
            self.logger.error("GitHub token not found in environment variables.")
            return None

        try:
            headers = {
                'Authorization': f'token {self.github_token}',
                'Accept': 'application/vnd.github.v3+json'
            }
            response = requests.get("https://api.github.com/user", headers=headers)
            response.raise_for_status()
            return response.json()['login']
        except requests.RequestException as e:
            self.logger.error(f"Error fetching GitHub username: {str(e)}")
            return None

    def is_developer(self, github_username: str) -> bool:
        """
        Check if the GitHub user is a member of the developer organization or team.
        """
        developer_organizations = ["anya-core-developers"]
        developer_teams = ["dev-team"]

        headers = {
            'Authorization': f'token {self.github_token}',
            'Accept': 'application/vnd.github.v3+json'
        }

        try:
            for org in developer_organizations:
                response = requests.get(f"https://api.github.com/orgs/{org}/members/{github_username}", headers=headers)
                if response.status_code == 204:
                    return True

                for team in developer_teams:
                    response = requests.get(f"https://api.github.com/orgs/{org}/teams/{team}/memberships/{github_username}", headers=headers)
                    if response.status_code == 200:
                        return True

            return False
        except requests.RequestException as e:
            self.logger.error(f"Error checking developer membership: {str(e)}")
            return False

    def encrypt_user_data(self, data: Dict[str, Any]) -> None:
        """
        Encrypt sensitive user data before storing.
        """
        for key, value in data.items():
            encrypted_value = self.cipher_suite.encrypt(str(value).encode())
            self.user_state.encrypted_data[key] = encrypted_value

    def decrypt_user_data(self, key: str) -> Optional[str]:
        """
        Decrypt and retrieve user data.
        """
        encrypted_value = self.user_state.encrypted_data.get(key)
        if encrypted_value:
            return self.cipher_suite.decrypt(encrypted_value).decode()
        return None

    def get_user_state(self) -> Dict[str, Any]:
        """
        Get the current state of the user, excluding sensitive information.
        """
        return {
            "github_username": self.user_state.github_username,
            "user_type": self.user_state.user_type
        }

    def initialize_user(self):
        """
        Initialize the user by identifying them and setting up their environment.
        """
        self.identify_user()
        setup_methods = {
            UserType.CREATOR: self.setup_creator_environment,
            UserType.DEVELOPER: self.setup_developer_environment,
            UserType.NORMAL: self.setup_normal_user_environment
        }
        setup_method = setup_methods.get(self.user_state.user_type, self.setup_normal_user_environment)
        setup_method()
        self.setup_project()

    def setup_creator_environment(self):
        self.logger.info("Setting up creator environment")
        # Implement creator-specific setup

    def setup_developer_environment(self):
        self.logger.info("Setting up developer environment")
        # Implement developer-specific setup

    def setup_normal_user_environment(self):
        self.logger.info("Setting up normal user environment")
        # Implement normal user setup

    def setup_project(self):
        """
        Set up the project using the ProjectSetup class.
        """
        project_setup = ProjectSetup(self.user_state.user_type, self.get_user_state())
        project_setup.setup()

    # Removed methods as they are now handled in ProjectSetup:
    # def setup_common_environment(self):
    # def setup_creator_project(self):
    # def setup_developer_project(self):
    # def setup_normal_user_project(self):
