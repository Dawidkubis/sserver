#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate comrak;
extern crate serde;
extern crate serde_derive;
extern crate structopt;
extern crate toml;

#[macro_use]
mod response;
mod models;
mod routes;

use models::{Cli, Settings};
use rocket::config::{Config, Environment};
use std::{thread, time};
use structopt::StructOpt;

pub static SETTINGS_PATH: &'static str = "settings.toml";
pub static WWW: &'static str = "www";

lazy_static! {
	pub static ref SETTINGS: Settings = Settings::get()
		.expect(&format!("Unable to parse {}", SETTINGS_PATH));
}

fn main() {
	// get cmd args
	let opt = Cli::from_args();

	// rocket config
	let config = Config::build(Environment::Development)
		.port(match opt.port {
			Some(i) => i,
			None => 8000,
		})
		.unwrap();

	// git repo update
	thread::spawn(|| loop {
		thread::sleep(time::Duration::from_secs(60));
		match SETTINGS.git.update() {
			Ok(s) => println!("git repo update: status = {}", s.status),
			Err(e) => eprintln!("{:?}", e),
		}
	});

	// rocket server init
	rocket::custom(config)
		.mount("/", routes![routes::index, routes::path,])
		.register(catchers![routes::not_found,])
		.launch();
}
