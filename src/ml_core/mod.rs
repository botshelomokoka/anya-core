mod data_processor;
mod model_trainer;
mod optimizer;
mod predictor;
mod gorules;
mod data;

use gorules::{init_gorules, execute_rule};
use log::info;

pub fn initialize_modules() {
	// Initialize GoRules
	if let Err(e) = init_gorules("path/to/config") {
		eprintln!("Error initializing GoRules: {}", e);
	}
	
	pub fn process_transaction_data(file_path: &str) -> Result<(), String> {
		let data = data::load_data(file_path)?;
		data::process_data(data)
	}return;
	}

	info!("Modules initialized successfully");
}

pub fn execute_business_logic(rule: &str) {
	// Execute a business rule using GoRules
	match execute_rule(rule) {
		Ok(_) => info!("Rule executed successfully"),
		Err(e) => eprintln!("Error executing rule: {}", e),
	}
}