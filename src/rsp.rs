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
