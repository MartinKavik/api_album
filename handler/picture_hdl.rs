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
use chrono::{NaiveDateTime, DateTime};
use diesel::expression_methods::*;
use chrono::offset::Utc;

use crate::service_error;
use crate::picture_sch;

#[path="../model/picture.rs"]
mod picture;
#[path="../model/new_picture.rs"]
mod new_picture;
use new_picture::NewPicture;
#[path="../model/picture_by_id.rs"]
mod picture_by_id;
use picture_by_id::PictureById;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_picture (
	param: web::Path<(u32)>,
	pool: web::Data<Pool>
) -> Result<web::Json<PictureById>> {
    info!("get_picture");
	
	let id = param.into_inner() as i32;

	let connection: &PgConnection = &pool.get().unwrap();
	let result = picture_sch::picture::dsl::picture
		.find(id)
		.first::<picture::Picture>(&*connection);

	match result {
		Ok(p) => Ok(convert_picture_to_json(p, None)),
		Err(_err) => Err(service_error::ServiceError::NotFound.into())
	}
}

pub fn get_picture_thumb (
	param: web::Path<(u32)>,
	pool: web::Data<Pool>
) -> Result<web::Json<PictureById>> {
    info!("get_picture");
	
	let id = param.into_inner() as i32;

	let connection: &PgConnection = &pool.get().unwrap();
	let result = picture_sch::picture::dsl::picture
		.find(id)
		.first::<picture::Picture>(&*connection);

	match result {
		Ok(p) => {
			let res_reseize = reseize(p.data.clone());
			match res_reseize {
				Ok(data) => {
					Ok(convert_picture_to_json(p, Some(data)))
				},
				Err(_err) => Err(service_error::ServiceError::InternalServerError.into())
			}
		},
		Err(_err) => Err(service_error::ServiceError::NotFound.into())
	}
}


pub fn get_picture_ids (
	pool: web::Data<Pool>
) -> Result<web::Json<Vec<i32>>> {
    info!("get_picture_ids");
	let connection: &PgConnection = &pool.get().unwrap();
	let result = picture_sch::picture::dsl::picture
		.select(picture_sch::picture::id)
		.order(picture_sch::picture::date.desc())
		.load(&*connection);

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
		.map(|data_vec| first(data_vec))
		.map(|data_vec| transform(data_vec))
		.map(|result| insert(result.unwrap(), pool))
        .map(|result| HttpResponse::Ok().json(result.ok()))
        .map_err(|e| {
            error!("failed: {}", e);
            e
        })
}

fn upload(field: Field) -> impl Future<Item = Vec<u8>, Error = Error> {
    get_filedata_vec(field)
}

fn first(data: Vec<Vec<u8>>) -> Vec<u8> {
	let pic = data.first().unwrap();
	pic.clone()
}

fn reseize(data: String) -> Result<String, std::io::Error> {
	let res_decode = base64::decode(&data);
	//let decode = data.as_bytes();
	let decode = match res_decode {
		Err(_e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "decode")),
		Ok(r) => r
	};
	let res_img = image::load_from_memory(&decode);
	match res_img {
		Err(_e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "reseize")),
		Ok(img) => {
			let img_reseize = img.thumbnail(100, 100);
			let mut buf = Vec::new();
			let res_write = img_reseize.write_to(&mut buf, image::ImageOutputFormat::PNG);
			match res_write {
				Err(_e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "write")),
				_ => {
					let data_encoded = base64::encode(&buf);
					Ok(data_encoded)
				}
			}
		}
	}
}

fn transform(img: Vec<u8>) -> Result<NewPicture, image::ImageError> {
	let data = base64::encode(&img);
	let mut picture = NewPicture {
		data: data,
		model: None,
		date: SystemTime::now(),
		latitude: None,
		longitude: None
	};
	let res_exif = rexif::parse_buffer(&img);
	if res_exif.is_ok() {
		let entries = &res_exif.unwrap().entries;
		parse_meta(entries, &ExifTag::Model, &mut picture);
		parse_meta(entries, &ExifTag::DateTime, &mut picture);
		parse_meta(entries, &ExifTag::GPSLatitude, &mut picture);
		parse_meta(entries, &ExifTag::GPSLongitude, &mut picture);
	}
	Ok(picture)
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
					let res = NaiveDateTime::parse_from_str(&e.value_more_readable.clone(), "%Y:%m:%d %H:%M:%S");
					if res.is_ok() {
						let date = res.unwrap();
						let ts = date.timestamp();
						let secs = Duration::from_secs(ts as u64);
						let time: SystemTime = SystemTime::UNIX_EPOCH + secs;
						picture.date = time;
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

fn convert_picture_to_json(p: picture::Picture, data_resized: Option<String>) -> web::Json<PictureById> {
	let datetime: DateTime<Utc> = p.date.clone().into();
	let date_str = datetime.format("%Y:%m:%d %H:%M:%S").to_string();
	let data = match data_resized {
		Some(d) => d.clone(),
		None => p.data.clone()
	};
	let pic = PictureById {
		id: p.id,
		data: data,
		model: p.model.clone(),
		date: date_str,
		longitude: p.longitude.clone(),
		latitude: p.latitude.clone()
	};
	web::Json(pic)
}