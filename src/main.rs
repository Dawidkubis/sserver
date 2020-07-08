//mod response;
//use response::File;
mod settings;
use settings::Settings;

use std::{
	process::Command,
	path::{PathBuf, Path},
	env,
	thread,
	time,
};

use structopt::StructOpt;
use anyhow::Result;
use tiny_http::{Server, Response, Method};

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
}

fn main() {
	let cla = Cla::from_args();
	let set = Settings::get(cla.settings).unwrap();
	
	// git repo update
	thread::spawn(|| loop {
		thread::sleep(time::Duration::from_secs(1));
		match Command::new("git").arg("pull").output() {
			Ok(s) => {
				if cfg!(debug_assertions) || !s.status.success() {
					println!("{:?}", s)
				}
			}
			Err(e) => eprintln!("{:?}", e),
		}
	});

	// http server init
	let server = Server::http(format!("0.0.0.0:{}", cla.port)).unwrap();

	// handle requests
	// TODO
	for request in server.incoming_requests() {
		let mut response = Response::from_string("404: url doesn't exist");
		let url = request.url();

		if let Method::Get = request.method() {
			println!("{:?}", request.url());
		}

		request.respond(response);
	}

}
