use crate::response::File;
use crate::rsp::Rsp;
use crate::Cla;

use std::path::{Path, PathBuf};

use rocket::{State, catch, get, Request};
use anyhow::Result;

#[get("/")]
pub fn index(cla: State<Cla>) -> Result<File> {
	let rsp = Rsp::get(cla.settings)?;

	File::open(rsp.index)
}

#[get("/<path..>")]
pub fn path(cla: State<Cla>, path: PathBuf) -> Result<File> {
	let rsp = Rsp::get(cla.settings)?;

	if let Some(s) = rsp.get {
		for i in s.into_iter() {
			if path.as_path() == Path::new(&i.uri) {
				return File::open(i.file);
			}
		}
	}

	File::open(path)
}
