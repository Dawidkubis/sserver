use structopt::StructOpt;
use std::path::PathBuf;

/// Command line arguments representation
#[derive(StructOpt)]
pub struct Cli {
	/// skeleton
	#[structopt(parse(from_os_str))]
	pub skel: PathBuf,
	/// port
	#[structopt(short, long, default_value="8000")]
	pub port: u16,
	/// settings file
	#[structopt(short, long, parse(from_os_str), default_value="settings.toml")]
	pub settings: PathBuf
}
