use crate::OPT;

use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use anyhow::{Error, Result};
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{self, FromRequest};
use rocket::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Get {
	pub uri: String,
	pub file: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Rsp {
	pub index: PathBuf,
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
