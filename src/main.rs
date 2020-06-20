#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
mod response;
mod routes;
mod rsp;

use std::{
	process::Command,
	path::PathBuf,
	env,
	thread,
	time,
};

use lazy_static::lazy_static;
use rocket::{catchers, routes};
use structopt::StructOpt;

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

lazy_static! {
	pub static ref CLA: Cla = Cla::from_args();
}

fn main() {
	// port setting
	env::set_var("ROCKET_PORT", format!("{}", OPT.port));

	// keep_alive setting
	env::set_var("ROCKET_KEEP_ALIVE", "0");

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

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::path, routes::index])
		.launch();
}
