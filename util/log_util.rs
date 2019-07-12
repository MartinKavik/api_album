use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

use crate::env_util;

pub fn set_logger() -> bool {
	let path = env_util::get_var("LOG_FILE");
	let create = File::create(path);
	let file = match create {
		Ok(l) => l,
		Err(_e) => {
			println!("error creating log file");
			return false;
		}
	};
	let logger = WriteLogger::init(LevelFilter::Info, Config::default(), file);
	match logger {
		Ok(l) => l,
		Err(_e) => {
			println!("error creating logger");
			return false;
		}
	};
	return true;
}