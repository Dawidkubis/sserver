use crate::response::File;
use crate::rsp::Rsp;

use std::path::{Path, PathBuf};

use rocket::{catch, get, Request};

#[get("/")]
pub fn index() -> Option<File> {
	File::open(rsp.index).ok()
}

#[get("/<path..>")]
pub fn path(path: PathBuf) -> Option<File> {
	if let Some(s) = rsp.get {
		for i in s.into_iter() {
			if path.as_path() == Path::new(&i.uri) {
				return File::open(i.file).ok();
			}
		}
	}

	File::open(path).ok()
}
