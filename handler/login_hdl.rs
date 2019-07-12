use actix_web::{web, Result};
use log::{info};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager};
use diesel::*;
use diesel::dsl::exists;

use crate::token_util;
use crate::service_error;

#[path="../model/token.rs"]
mod token;
#[path="../model/login.rs"]
mod login;
#[path="../schema/user_sch.rs"]
mod user_sch;
#[path="../model/user.rs"]
mod user;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn login(
	login: web::Json<login::Login>,
	pool: web::Data<Pool>
) -> Result<web::Json<token::Token>>  {
	info!("login");
    //TODO : check in BDD
	let result = token_util::create_token(&login.email);
	match result {
		Ok(t) =>  Ok(web::Json(token::Token { token: t })),
		Err(_e) => Err(service_error::ServiceError::Unauthorized.into())
	}
}

fn user_exists(
	login: login::Login,
	pool: web::Data<Pool>
) ->  Result<bool>  {
	let sha256 = login.password;
	let connection: &PgConnection = &pool.get().unwrap();

	let res = select(exists(user_sch::user::dsl::user
		.filter(user_sch::user::email.eq(login.email)
			.and(user_sch::user::password.eq(sha256)))))
			.execute(connection);

	match res {
		Ok(_r) => Ok(true),
		Err(_e) => Err(service_error::ServiceError::InternalServerError.into())
	}
}