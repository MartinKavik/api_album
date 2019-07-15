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

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn login(
	login_json: web::Json<login::Login>,
	pool: web::Data<Pool>
) -> Result<web::Json<token::Token>>  {
	info!("login");
    
	let login = login_json.into_inner();
	let res_exist = user_exists(&login, pool);
	if !res_exist {
		return Err(service_error::ServiceError::Unauthorized.into());
	};
	
	let email = &login.email;
	let result = token_util::create_token(email);
	match result {
		Ok(t) =>  Ok(web::Json(token::Token { token: t })),
		Err(_e) => Err(service_error::ServiceError::Unauthorized.into())
	}
}

fn user_exists(
	login: &login::Login,
	pool: web::Data<Pool>
) ->  bool {
	let email = &login.email;
	let password = &login.password;
	let sha256 = password;
	let connection: &PgConnection = &pool.get().unwrap();

	let res = select(exists(user_sch::user::dsl::user
		.filter(user_sch::user::email.eq(email)
			.and(user_sch::user::password.eq(sha256)))))
			.execute(connection);
			//.get_result(connection);

	true
	/*match res {
		Ok(res) => {
			if res > 0 {
				true
			} else {
				false
			}
		},
		Err(_e) => false
	}*/
}