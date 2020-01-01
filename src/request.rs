use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use anyhow::{Result, Error};
use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::io;

static SETTINGS: &'static str = "settings.toml";

#[derive(Debug, Deserialize)]
pub struct Git {
	pub url: String,
	pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
	pub index: String,
	pub git: Git,
}

impl Settings {
	pub fn get() -> Result<Self> {
		let s = &read_to_string(SETTINGS)?;
		Ok(toml::from_str::<Self>(&s)?)
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Settings {
	type Error = Error;

	fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match Self::get() {
			Ok(s) => Outcome::Success(s),
			Err(e) => Outcome::Failure((Status::InternalServerError, anyhow!("{}", e)))
		}
	}
}

