use crate::{SETTINGS_PATH, WWW};
use anyhow::{Context, Result};
use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::process::{Command, Output};
use structopt::StructOpt;

/// Command line arguments representation
#[derive(StructOpt)]
pub struct Cli {
	/// port
	#[structopt(short, long)]
	pub port: Option<u16>,
}

/// Representation of a git repo
#[derive(Debug, Deserialize)]
pub struct Git {
	/// git repo url - ssh or http
	pub url: String,
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
}

impl Settings {
	/// parses SETTINGS_PATH into Settings
	pub fn get() -> Result<Self> {
		let s = &read_to_string(SETTINGS_PATH)?;
		Ok(toml::from_str::<Self>(&s)?)
	}
}
