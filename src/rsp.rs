use crate::SETTINGS;
use anyhow::{Error, Result};
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{self, FromRequest};
use rocket::Request;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Get {
	pub uri: String,
	pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Rsp {
	pub get: Option<Vec<Get>>,
}

impl Rsp {
	fn get<S>(path: S) -> Result<Self>
	where
		S: AsRef<Path>,
	{
		let s = &read_to_string(path)?;
		Ok(toml::from_str::<Self>(s)?)
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Rsp {
	type Error = Error;

	fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match &SETTINGS.responses {
			Some(s) => match Rsp::get(SETTINGS) {
				Ok(s) => Outcome::Success(s),
				Err(e) => Outcome::Failure((Status::InternalServerError, e)),
			},
			None => Outcome::Success(Rsp { get: None }),
		}
	}
}
