use structopt::StructOpt;

/// Command line arguments representation
#[derive(StructOpt)]
pub struct Cli {
	/// port
	#[structopt(short, long)]
	pub port: Option<u16>,
}

