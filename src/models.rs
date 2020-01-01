use crate::{SETTINGS_PATH, WWW};
use anyhow::{Context, Result};
use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::process::{Command, ExitStatus};
use structopt::StructOpt;

/// Specify which port to run on
/// `8000` is the default
#[derive(StructOpt)]
pub struct Cli {
	/// the port on which to run
	#[structopt(short, long)]
	pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Git {
	pub url: String,
}

impl Git {
	pub fn update(&self) -> Result<ExitStatus> {
		Command::new("git")
			.arg("pull")
			.current_dir(WWW)
			.spawn()
			.context("failed to spawn command")?
			.wait()
			.context("failed to spawn command")
	}

	pub fn clone(&self) -> Result<ExitStatus> {
		Command::new("git")
			.arg("clone")
			.arg(&self.url)
			.arg(WWW)
			.spawn()
			.context("failed to spawn command")?
			.wait()
			.context("failed to spawn command")
	}
}

#[derive(Debug, Deserialize)]
pub struct Settings {
	pub index: String,
	pub git: Git,
}

impl Settings {
	pub fn get() -> Result<Self> {
		let s = &read_to_string(SETTINGS_PATH)?;
		Ok(toml::from_str::<Self>(&s)?)
	}
}
