use anya_enterprise::ml::{InternalAIEngine, GitHubIntegrator, MockGitHubIntegrator, Issue};
use tokio::runtime::Runtime;

#[test]
fn test_internal_ai_engine() {
	let rt = Runtime::new().unwrap();
	let ai_engine = InternalAIEngine::init().unwrap();
	let result = rt.block_on(ai_engine.perform_research());
	assert!(result.is_ok());
}

#[test]
fn test_submit_upgrade_request_with_mock() {
	let rt = Runtime::new().unwrap();
	let mut mock_github_integrator = MockGitHubIntegrator::new();
	mock_github_integrator.expect_create_issue()
		.with(predicate::eq("fake_repo/fake_project"), predicate::always())
		.times(1)
		.returning(|_, _| Ok(()));

	let ai_engine = InternalAIEngine::init().unwrap();
	let result = rt.block_on(ai_engine.submit_upgrade_request(
		"fake_repo/fake_project",
		"Test Upgrade Request",
		"This is a test upgrade request.",
	));
	assert!(result.is_ok());
}

#[test]
fn test_submit_upgrade_request() {
	let rt = Runtime::new().unwrap();
	let ai_engine = InternalAIEngine::init().unwrap();
	let result = rt.block_on(ai_engine.submit_upgrade_request(
		"fake_repo/fake_project",
		"Test Upgrade Request",
		"This is a test upgrade request.",
	));
	assert!(result.is_err()); // Expecting an error due to fake token and repo
}