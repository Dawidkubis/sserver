use crate::models::Settings;
use rocket::Request;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index(s: Settings) -> String {
	String::from("wip")
}

#[get("/<path..>")]
pub fn path(path: PathBuf, s: Settings) -> String {
	String::from("wyp")
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
