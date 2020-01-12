use crate::response::File;
use crate::rsp::Rsp;
use crate::{path, SETTINGS, WWW};
use anyhow::Result;
use rocket::Request;
use std::path::{Path, PathBuf};

/// Index
#[get("/")]
pub fn index() -> File {
	let path = path!(&SETTINGS.index);
	File::open(path).unwrap()
}

#[get("/<path..>")]
pub fn path(path: PathBuf, rsp: Rsp) -> Option<File> {
	if let Some(s) = rsp.response {
		for i in s.into_iter() {
			if path == Path::new(&i.uri) {
				let p = Path::new(WWW).join(i.file);
				return File::open(p).ok();
			}
		}
	}
	let p = Path::new(WWW).join(path);
	File::open(p).ok()
}

// Catchers
// TODO

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
