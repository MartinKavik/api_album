use diesel::r2d2::{Pool, ConnectionManager};
use diesel::pg::PgConnection;
use log::error;

use crate::env_util;

pub fn get_pool() -> Option<Pool<ConnectionManager<PgConnection>>>
{
	let db_url = env_util::get_var("DATABASE_URL");
	let manager = ConnectionManager::<PgConnection>::new(db_url);
	let res_pool = r2d2::Pool::builder().build(manager);
	match res_pool {
		Ok(l) => Some(l),
		Err(_e) => {
			error!("error connecting database");
			None
		}
	}
}