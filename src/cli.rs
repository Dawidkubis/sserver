use structopt::StructOpt;
use std::path::PathBuf;

/// Command line arguments representation
#[derive(StructOpt)]
pub struct Cli {
	/// port
	#[structopt(short, long)]
	pub port: Option<u16>,
	/// settings file
	#[structopt(short, long)]
	pub settings: Option<PathBuf>
}
