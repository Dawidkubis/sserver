use crate::response::File;
use crate::rsp::Rsp;
use anyhow::Result;
use rocket::{catch, get, Request};
use std::path::{Path, PathBuf};

#[get("/<path..>")]
pub fn path(path: PathBuf, rsp: Result<Rsp>) -> Option<File> {
	let rsp = match rsp {
		Ok(s) => s,
		Err(e) => {
			eprintln!("{:?}", e);
			return None;
		}
	};

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
