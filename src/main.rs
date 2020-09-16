//mod response;
//use response::File;
mod settings;
use settings::*;

use std::{
	process::Command,
	path::{PathBuf, Path},
	env,
	thread,
	time,
	fs::{read_to_string, File},
};

use structopt::StructOpt;
use anyhow::Result;
use tiny_http::{Server, Response, Method};
use simplelog::*;
use log::*;

// TODO logging

/// Command line arguments representation
#[derive(StructOpt)]
pub struct Cla {
	/// settings file
	#[structopt(short, long, parse(from_os_str), default_value = "settings.toml")]
	pub settings: PathBuf,
	/// port
	#[structopt(short, long, default_value = "8000")]
	pub port: u16,
	/// log file
	#[structopt(short, long)]
	pub log: Option<PathBuf>,
	/// refresh time
	#[structopt(short, long, default_value = "5")]
	pub refresh: u64,
}

//fn get_template() -> String {

//}

fn generate_response(uri: &str, set: Vec<Get>) -> Response<File> {

	// if route specified in settings
	let file = if let Some(s) = set.iter() 
		.find(|x| x.uri == uri) 
	{
		// return contents of the specified file
		Some(File::open(s.file))
	} else {

		let path = Path::new(uri);

		// return any file with specified path	
		if path.exists() {
			Some(File::open(path))
		} else {
			None
		}
	};
	
}


fn main() {
	// get command line arguments
	let cla = Cla::from_args();

	// read settings file
	let set = Settings::get(cla.settings).unwrap();

	// shadow set + fletten
	let set = match set.get {
		Some(s) => s,
		None => vec![],
	};

	// init logger
	// TODO other logging options
	// TODO log to file
	let logger = TermLogger::new(LevelFilter::Off, Config::default(), TerminalMode::Mixed);
	
	// git repo update
	thread::spawn(|| loop {
		// sleep for a second
		thread::sleep(time::Duration::from_secs(cla.refresh));
		
		// call "git pull"
		match Command::new("git").arg("pull").output() {
			Ok(s) => {
				// print result if return code != 0
				if cfg!(debug_assertions) || !s.status.success() {
					error!("{:?}", s)
				}
			}
			// return info if unable to spawn subprocess
			Err(e) => error!("{:?}", e),
		}
	});

	// http server init
	let server = Server::http(format!("0.0.0.0:{}", cla.port)).unwrap();

	// handle requests
	// TODO
	for request in server.incoming_requests() {
		let url = request.url();

		// ask for method
		match request.method() {
			// if get
			// TODO into a function for easier recycle
			Method::Get => {
			},
			// TODO other methods
			_ => (),
		}

		request.respond(response);
	}

}
