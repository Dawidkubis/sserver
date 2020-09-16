use std::path::PathBuf;
use structopt::StructOpt;

/// Command line arguments representation
#[derive(StructOpt)]
pub struct Cli {
	/// template file (for markdown)
	#[structopt(parse(from_os_str))]
	pub skel: PathBuf,
	/// port
	#[structopt(short, long, default_value = "8000")]
	pub port: u16,
	/// settings file
	#[structopt(short, long, parse(from_os_str), default_value = "settings.toml")]
	pub settings: PathBuf,
	/// refresh rate (in seconds)
	#[structopt(short, long, default_value="5")]
	pub refresh: u64,
}
