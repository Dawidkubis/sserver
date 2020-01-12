use crate::{SETTINGS_PATH, WWW};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::{read_to_string, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[macro_export]
macro_rules! path {
	($e:expr) => {
		[WWW, $e].iter().collect::<PathBuf>()
	};
}

/// Representation of a git repo
#[derive(Debug, Deserialize)]
pub struct Git {
	/// git repo url - ssh or http
	pub url: String,
	/// branch
	pub branch: String,
}

impl Git {
	/// update the repo - clone and pull
	/// since git does nothing when cloning
	/// already cloned repo this is fine
	/// uses WWW
	pub fn update(&self) -> Result<Output> {
		self.clone()?;

		Command::new("git")
			.arg("pull")
			.current_dir(WWW)
			.output()
			.context("failed to spawn command")
	}

	/// clone the repo
	/// uses WWW
	fn clone(&self) -> Result<Output> {
		Command::new("git")
			.arg("clone")
			.arg("--single-branch")
			.arg("-b")
			.arg(&self.branch)
			.arg(&self.url)
			.arg(WWW)
			.output()
			.context("failed to spawn command")
	}
}

#[derive(Debug, Deserialize)]
/// Representation of SETTINGS_PATH file
pub struct Settings {
	/// index of the page
	pub index: String,
	/// name of skeleton file
	pub skeleton: String,
	/// list of responses
	pub response: Option<String>,
	/// git
	pub git: Git,
}

impl Settings {
	/// parses SETTINGS_PATH into Settings
	pub fn get() -> Result<Self> {
		let s = &read_to_string(SETTINGS_PATH)?;
		let s = toml::from_str::<Self>(&s)?;

		// handle exceptions
		// - responses file doesnt exist (if specified)
		// or is invalid

		if Path::new(WWW).is_dir() {
			remove_dir_all(WWW)?;
		}

		let o = s.git.clone()?;

		if !o.status.success() {
			return Err(anyhow!("git error : {:?}", o));
		}

		if !path!(&s.index).is_file() {
			return Err(anyhow!("index file not found : {:?}", s.index));
		}

		if !path!(&s.skeleton).is_file() {
			return Err(anyhow!("skeleton file not found : {}", s.skeleton));
		}

		Ok(s)
	}
}
