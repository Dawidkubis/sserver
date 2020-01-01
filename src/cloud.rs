use crate::markdown;
use crate::request::Settings;
use crate::response::{md, HTML};
use anyhow::Result;
use rocket::response::content::Html;
use std::fs::File;
use std::path::PathBuf;

/// cloud
#[get("/cloud/<path..>")]
pub fn cloud(s: Settings, path: PathBuf) -> Result<HTML> {
	//TODO
}

/// returns a vector tree of files
/// works recursively
fn listdir(s: &str) -> Result<String> {
	//TODO
}
