use libc::{c_char, c_int};
use std::ffi::CString;
use std::ptr;

#[link(name = "gorules")]  // Link to the GoRules library
extern "C" {
	fn gorules_init(config: *const c_char) -> c_int;
	fn gorules_execute(rule: *const c_char) -> c_int;
	fn gorules_load_rules(file: *const c_char) -> c_int;  // Add this function
}

pub fn init_gorules(config: &str) -> Result<(), String> {
	let c_config = CString::new(config).map_err(|e| e.to_string())?;
	let result = unsafe { gorules_init(c_config.as_ptr()) };
	if result == 0 {
		Ok(())
	} else {
		Err("Failed to initialize GoRules".into())
	}
}

pub fn execute_rule(rule: &str) -> Result<(), String> {
	let c_rule = CString::new(rule).map_err(|e| e.to_string())?;
	let result = unsafe { gorules_execute(c_rule.as_ptr()) };
	if result == 0 {
		Ok(())
	} else {
		Err("Failed to execute rule".into())
	}
}

pub fn load_rules(file: &str) -> Result<(), String> {  // Add this function
	let c_file = CString::new(file).map_err(|e| e.to_string())?;
	let result = unsafe { gorules_load_rules(c_file.as_ptr()) };
	if result == 0 {
		Ok(())
	} else {
		Err("Failed to load rules".into())
	}
}
