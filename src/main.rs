/* mod response; */
/* mod routes; */
/* mod rsp; */

use std::{
	process::Command,
	path::PathBuf,
	env,
	thread,
	time,
};

use structopt::StructOpt;
use tiny_http::{Server, Response};

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
	// handle command line arguments
	let cla = Cla::from_args();
	
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

	// initialize server
	let server = Server::http(format!("0.0.0.0:{}", cla.port)).unwrap();

	// handle requests
	for request in server.incoming_requests() {
		request.respond(Response::from_string("nigger"));
	}
	
}

