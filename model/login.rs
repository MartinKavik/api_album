use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Login {
	pub email: String,
	pub password: String
}