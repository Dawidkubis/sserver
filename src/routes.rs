use crate::response::{md, HTML};
use rocket::response::content::Html;
use rocket::response::NamedFile;
use rocket::Request;
use std::fs::read_to_string;

/// Index
#[get("/")]
pub fn index() -> String {
	String::from("wip")
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
