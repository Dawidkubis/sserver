use serde::Deserialize;
use rocket::request::{FromRequest, self};
use rocket::Request;
use rocket::http::Status;
use rocket::request::Outcome;
use anyhow::{Error, Result};
use std::path::Path;
use std::fs::read_to_string;
use crate::SETTINGS;

#[derive(Debug, Deserialize)]
pub struct Route {
	pub uri: String,
	pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Rsp {
	pub response: Vec<Route>
}

impl Rsp {
	fn get<S>(path: S) -> Result<Self>
	where
		S: AsRef<Path>
	{
		let s = &read_to_string(path)?;
		Ok(toml::from_str::<Self>(s)?)
	}
}

// FUCK FUCK FUCK this isnt working at all
impl<'a, 'r> FromRequest<'a, 'r> for Option<Rsp> {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Option<Self>, Self::Error> {
    	match SETTINGS.response {
    		Some(s) => match Rsp::get(s) {
    			Ok(s) => Outcome::Success(s),
    			Err(e) => Outcome::Failure((Status::new(500), e))
    		}
    		None => Outcome::Success(None),
    	}
    }
}
