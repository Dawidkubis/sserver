use crate::response::File;
use crate::rsp::Rsp;

use std::path::{Path, PathBuf};

use rocket::{catch, get, Request};

#[get("/")]
pub fn index(rsp: Rsp) -> Option<File> {
	File::open(rsp.index).ok()
}

#[get("/<path..>")]
pub fn path(path: PathBuf, rsp: Rsp) -> Option<File> {
	if let Some(s) = rsp.get {
		for i in s.into_iter() {
			if path == Path::new(&i.uri) {
				File::open(i.file).ok();
			}
		}
	}

	File::open(path).ok()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
