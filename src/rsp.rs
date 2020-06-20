use crate::CLA;

use std::{fs::read_to_string, path::Path};

use anyhow::Result;
use rocket::{
	http::Status,
	request::{self, Outcome, FromRequest},
	Request
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Get {
	pub uri: String,
	pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Rsp {
	pub index: String,
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
		match Rsp::get(&OPT.settings) {
			Ok(s) => Outcome::Success(s),
			Err(e) => Outcome::Failure((Status::InternalServerError, e)),
		}
	}
}
