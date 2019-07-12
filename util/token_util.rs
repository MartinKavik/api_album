use jsonwebtoken::{encode, decode, Header, Validation, errors};
use chrono::{Local};
use log::info;

use crate::env_util;

#[path="../model/claims.rs"]
mod claims;

pub fn create_token(email: &str) -> Result<String, errors::Error> {
    let claims = claims::Claims::with_email(&email);
    encode(&Header::default(), &claims, env_util::get_var("JWT_SECRET").as_ref())
}

pub fn validate_token(token: &str) -> bool {
	let result = decode::<claims::Claims>(token, env_util::get_var("JWT_SECRET").as_ref(), &Validation::default());
	if !result.is_ok() {
		info!("Token not valid");
		return false;
	} else {
		let exp = result.ok().unwrap().claims.exp;
		if Local::now().timestamp() > exp {
			info!("Token expired");
			return false;
		}
	}
    true
}
