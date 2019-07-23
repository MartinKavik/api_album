use actix_web::{web, Result, error, Error, HttpResponse };
use log::{info, error};
use diesel::r2d2::{ConnectionManager};
use diesel::pg::PgConnection;
use diesel::{QueryDsl, RunQueryDsl};
use actix_multipart::{Multipart, Field, MultipartError};
use futures::{future, Future, Stream};
use base64;
use rexif::{ExifTag, ExifEntry};
use std::time::{SystemTime, Duration};
use chrono::{DateTime};

use crate::service_error;
use crate::picture_sch;

#[path="../model/picture.rs"]
mod picture;
#[path="../model/new_picture.rs"]
mod new_picture;
use new_picture::NewPicture;

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
		.map(|picture| insert(picture, pool))
        .map(|result| HttpResponse::Ok().json(result.ok()))
        .map_err(|e| {
            error!("failed: {}", e);
            e
        })
}

fn upload(field: Field) -> impl Future<Item = Vec<u8>, Error = Error> {
    get_filedata_vec(field)
}

fn transform(data: Vec<Vec<u8>>) -> NewPicture {
	let first = data.first().unwrap();
	let data = base64::encode(first);
	let mut picture = NewPicture {
		data: data,
		model: None,
		date: SystemTime::now(),
		latitude: None,
		longitude: None
	};
	let res_exif = rexif::parse_buffer(&first);
	if res_exif.is_ok() {
		let entries = &res_exif.unwrap().entries;
		parse_meta(entries, &ExifTag::Model, &mut picture);
		parse_meta(entries, &ExifTag::DateTime, &mut picture);
		parse_meta(entries, &ExifTag::GPSLatitude, &mut picture);
		parse_meta(entries, &ExifTag::GPSLongitude, &mut picture);
	}
	picture
}

fn parse_meta(entries: &Vec<ExifEntry>, tag: &ExifTag, picture: &mut NewPicture) 
{
	let mut iter = entries.into_iter();
	let opt = iter.find(| &x| x.tag.eq(tag));
	match opt {
		Some(e) => {
			match tag {
				ExifTag::Model => picture.model = Some(e.value_more_readable.clone()),
				ExifTag::DateTime => {
					println!("{}", e.value_more_readable);
					let res = DateTime::parse_from_str(&e.value_more_readable.clone(), "%Y:%m:%d %H:%M:%S");
					if res.is_ok() {
						let date = res.unwrap();
						let ts = date.timestamp();
						//let dur = SystemTime::from_secs(ts as u64);
						//picture.date = ts;
					}
				}
				ExifTag::GPSLatitude => picture.latitude = Some(e.value_more_readable.clone()),
				ExifTag::GPSLongitude => picture.longitude = Some(e.value_more_readable.clone()),
				_ => ()
			}
		},
		None => ()
	};
}

fn insert(picture: NewPicture, pool: web::Data<Pool>) -> Result<bool> {
	let con: &PgConnection = &pool.get().unwrap();
    let res = diesel::insert_into(picture_sch::picture::table)
		.values(&picture)
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
            error!("bytes receive failed, {:?}", e);
            error::ErrorInternalServerError(e)
        })
	)
}