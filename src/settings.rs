use serde::Deserialize;
use std::process::{Command, Output};
use std::fs::read_to_string;
use anyhow::{Result, Context};
use crate::{WWW, SETTINGS_PATH};

// TODO make the impl's better
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
	/// git
	pub git: Git,
	/// index of the page
	pub index: String,
	/// name of skeleton file
	pub skeleton: String,
	/// should all the files be served?
	pub serve_all: bool,
	/// list of responses
	pub response: Option<String>,
}

impl Settings {
	/// parses SETTINGS_PATH into Settings
	pub fn get() -> Result<Self> {
		let s = &read_to_string(SETTINGS_PATH)?;
		let s = toml::from_str::<Self>(&s)?;

		// handle exceptions
		// 1 - skeleton doesnt exist
		// 2 - git url doesnt exist
		// 3 - git branch doesnt exist
		// 4 - responses file doesnt exist (if specified)
		// or is invalid
		// 5 - index doesnt exist
		Ok(s)
	}
}
