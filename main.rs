#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use actix_web::{HttpServer, App, web};

#[path="./middleware/jwt_auth_mdl.rs"]
mod jwt_auth_mdl;
#[path="./handler/login_hdl.rs"]
mod login_hdl;
#[path="./handler/picture_hdl.rs"]
mod picture_hdl;
#[path="./util/log_util.rs"]
mod log_util;
#[path="./util/db_util.rs"]
mod db_util;

#[path="./util/token_util.rs"]
pub mod token_util;
#[path="./util/env_util.rs"]
pub mod env_util;
#[path="./errors/service_error.rs"]
pub mod service_error;
#[path="./schema/picture_sch.rs"]
pub mod picture_sch;

fn main() -> std::io::Result<()> {
	
	let err = std::io::Error::from(std::io::ErrorKind::Other);

	dotenv().ok();
	let sys = actix_rt::System::new("rust_api");
	
	//Logger
	if !log_util::set_logger() {
		return Err(err);
	}

	//Database
	let pool_opt = db_util::get_pool();
	let pool = match pool_opt {
		Some(p) => p,
		None => return Err(err)
	};

    HttpServer::new(move || {
        App::new()
			.data(pool.clone())
			.wrap(jwt_auth_mdl::JwtAuth)
			.service(
				web::resource("/picture/{id}")
            		.route(web::get().to(picture_hdl::get_picture))
				)
			.service(
				web::resource("/picture")
            		.route(web::post().to_async(picture_hdl::post_picture))
					.route(web::get().to(picture_hdl::get_pictures_ids))
				)
			.service(
				web::resource("/login")
            		.route(web::post().to(login_hdl::login))
				)
    }).bind("127.0.0.1:8080")
    .unwrap()
    .start();

	sys.run()
}
