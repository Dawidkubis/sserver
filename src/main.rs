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
use std::process::Command;

/// Specify which port to run on
/// `8000` is the default
#[derive(StructOpt)]
struct Cli {
	/// the port on which to run
	#[structopt(short, long)]
	port: Option<u16>,
}

fn git_update(){
	
}

fn main() {
	let opt = Cli::from_args();

	let config = Config::build(Environment::Development)
		.port(match opt.port {
			Some(i) => i,
			None => 8000,
		})
		.unwrap();

	rocket::custom(config)
		.mount("/", routes![routes::index, routes::path,])
		.register(catchers![routes::not_found,])
		.launch();
}
