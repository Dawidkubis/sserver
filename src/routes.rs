use rocket::Request;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index() -> String {
	String::from("wip")
}

#[get("/<path..>")]
pub fn path(path: PathBuf) -> String {
	String::from("wyp")
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
