#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
extern crate comrak;
extern crate serde;
extern crate serde_derive;
extern crate structopt;
extern crate toml;

#[macro_use]
mod response;
mod request;
mod routes;

use rocket::config::{Config, Environment};
use structopt::StructOpt;
use std::thread;
use std::time;
use std::path::Path;
use std::process::Command;

static WWW: &'static str = "www";

/// Specify which port to run on
/// `8000` is the default
#[derive(StructOpt)]
struct Cli {
	/// the port on which to run
	#[structopt(short, long)]
	port: Option<u16>,
}

fn git_update() {
	loop {
		thread::sleep(time::Duration::from_secs(60));

		let settings = match request::Settings::get() {
			Ok(s) => s,
			Err(e) => {
				eprintln!("{:?}", e);
				continue
			},
		};
		
		let path = Path::new()
		
	}	
}

fn main() {
	let opt = Cli::from_args();

	let config = Config::build(Environment::Development)
		.port(match opt.port {
			Some(i) => i,
			None => 8000,
		})
		.unwrap();
	
	thread::spawn(git_update);

	rocket::custom(config)
		.mount("/", routes![routes::index, routes::path,])
		.register(catchers![routes::not_found,])
		.launch();
}
