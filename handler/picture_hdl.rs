use actix_web::{web, Result, error, Error, HttpResponse };
use log::{info};
use diesel::r2d2::{ConnectionManager};
use diesel::pg::PgConnection;
use diesel::{QueryDsl, RunQueryDsl};
use actix_multipart::{Multipart, Field, MultipartError};
use futures::{future, Future, Stream};
use base64;

use crate::service_error;
use crate::picture_sch;

#[path="../model/picture.rs"]
mod picture;
#[path="../model/new_picture.rs"]
mod new_picture;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_picture (
	param: web::Path<(u32)>,
	pool: web::Data<Pool>
) -> Result<web::Json<picture::Picture>> {
    info!("get_picture");
	
	let id = param.into_inner() as i32;

	let connection: &PgConnection = &pool.get().unwrap();
	let result = picture_sch::picture::dsl::picture
		.find(id)
		.first::<picture::Picture>(&*connection);

	match result {
		Ok(p) => Ok(web::Json(p)),
		Err(_err) => Err(service_error::ServiceError::NotFound.into())
	}
}

pub fn post_picture(
	multipart: Multipart,
	pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = Error> {
	info!("post_picture");

	multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| upload(field).into_stream())
        .flatten()
        .collect()
		.map(|data_vec| transform(data_vec))
		.map(|data| insert(data, pool))
        .map(|result| HttpResponse::Ok().json(result.ok()))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}

fn upload(field: Field) -> impl Future<Item = Vec<u8>, Error = Error> {
    get_filedata_vec(field)
}

fn transform(data: Vec<Vec<u8>>) -> String {
    let first = data.first().unwrap();
	base64::encode(first)
}

fn insert(data: String, pool: web::Data<Pool>) -> Result<bool> {
	let con: &PgConnection = &pool.get().unwrap();
    let new_picture = new_picture::NewPicture {
		data: data
	};

	let res = diesel::insert_into(picture_sch::picture::table)
		.values(&new_picture)
		.execute(con);

	match res {
		Ok(_r) => Ok(true),
		Err(_e) => Err(service_error::ServiceError::InternalServerError.into())
	}
}

fn get_filedata_vec(field: Field) -> Box<Future<Item = Vec<u8>, Error = Error>> {
	Box::new(
        field.fold(Vec::new(), move |mut acc : Vec<u8>, bytes| {
            acc.append(bytes.to_vec().as_mut());
           	let rt: Result<Vec<u8>, MultipartError> = Ok(acc);
			future::result(rt)
        })
        .map_err(|e| {
            println!("bytes receive failed, {:?}", e);
            error::ErrorInternalServerError(e)
        })
	)
}