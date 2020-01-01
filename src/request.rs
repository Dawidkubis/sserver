use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::io;

#[derive(Debug, Deserialize)]
pub struct Settings {
	pub data_path: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Settings {
	type Error = io::Error;

	fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, io::Error> {
		match read_to_string("settings.toml") {
			Ok(s) => match toml::from_str::<Settings>(&s) {
				Ok(s) => Outcome::Success(s),
				Err(e) => Outcome::Failure((Status::InternalServerError, io::Error::from(e))),
			},
			Err(e) => Outcome::Failure((Status::InternalServerError, e)),
		}
	}
}
