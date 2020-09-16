use std::{fs::read_to_string, path::{Path, PathBuf}};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Get {
	pub uri: String,
	pub file: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
	pub get: Option<Vec<Get>>,
}

impl Settings {
	pub fn get<S>(path: S) -> Result<Self>
	where
		S: AsRef<Path>,
	{
		let s = &read_to_string(path)?;
		Ok(toml::from_str::<Self>(s)?)
	}
}
