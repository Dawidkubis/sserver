use crate::response::{md, HTML};
use rocket::response::content::Html;
use rocket::response::NamedFile;
use rocket::Request;
use std::fs::read_to_string;

/// macro for opening static files
macro_rules! open {
	($e:expr) => {
		NamedFile::open($e).unwrap()
	};
}

macro_rules! open_md {
	($e:expr) => {
		Html(md(&read_to_string($e).unwrap()))
	};
}

/// Index
#[get("/")]
pub fn index() -> HTML {
	open_md!("frontend/index.md")
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
