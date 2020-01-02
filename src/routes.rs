use crate::response::File;
use crate::{SETTINGS, WWW};
use rocket::Request;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index() -> File {
	let path = [WWW, &SETTINGS.index].iter().collect::<PathBuf>();
	File::open(path).unwrap()
}

#[get("/<path..>")]
pub fn path(path: PathBuf) -> Option<File> {
	let mut p = PathBuf::new();
	p.push(WWW);
	p.push(path);
	File::open(p).ok()
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
