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

/// Css, I'll be using only one stylesheet
#[get("/css")]
pub fn css() -> NamedFile {
	open!("frontend/main.css")
}

/// favicon - some stupid shit if I have to guess
#[get("/favicon.ico")]
pub fn favicon() -> NamedFile {
	open!("frontend/favicon.ico")
}

/// git - probably a forward to some other service
#[get("/git")]
pub fn git() -> &'static str {
	"working on it..."
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
