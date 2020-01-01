use rocket::Request;
use crate::request::Settings;

/// Index
#[get("/")]
pub fn index(s: Settings) -> String {
	String::from("wip")
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
