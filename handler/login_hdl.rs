use actix_web::{web, Result};
use log::*;

use crate::token_util;
use crate::service_error;

#[path="../model/token.rs"]
mod token;
#[path="../model/login.rs"]
mod login;

pub fn login(login: web::Json<login::Login>) -> Result<web::Json<token::Token>>  {
	info!("login");
    //TODO : check in BDD
	let result = token_util::create_token(&login.email);
	match result {
		Ok(t) =>  Ok(web::Json(token::Token { token: t })),
		Err(_e) => Err(service_error::ServiceError::Unauthorized.into())
	}
}