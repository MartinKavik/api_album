use std::env;
use log::error;

pub fn get_var(name: &str) -> String {
	let result = env::var(name);
	match result {
		Ok(t) => t,
		Err(_e) => {
			error!("env var {} not found", name);
			"".to_string()
		}
	}
}
