use serde::Deserialize;
use rocket::request::{FromRequest, self};
use rocket::Request;
use anyhow::{Error, Result};
use crate::SETTINGS;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct Rsp {
	pub uri: String,
	pub file: String,
}

impl Rsp {
	fn get() -> Result<Self> {
		let s = &read_to_string(SETTINGS.response?);
		Ok(toml::from_str::<Self>(s)?)
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Rsp {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {

    }
}
