use crate::response::File;
use crate::rsp::Rsp;
use crate::WWW;
use anyhow::Result;
use rocket::{catch, get, Request};
use std::path::{Path, PathBuf};

/// Index
#[get("/")]
pub fn index() -> File {
	let path = path!(&SETTINGS.index);
	File::open(path).unwrap()
}

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
				let p = Path::new(WWW).join(i.file);
				return File::open(p).ok();
			}
		}
	}

	if SETTINGS.serve_all {
		let p = Path::new(WWW).join(path);
		return File::open(p).ok();
	}

	None
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
