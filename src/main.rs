#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate comrak;
extern crate serde;
extern crate structopt;
extern crate toml;

#[macro_use]
mod response;
mod models;
mod routes;
mod settings;

use models::Cli;
use std::{thread, time, env};
use structopt::StructOpt;
use settings::Settings;

pub const SETTINGS_PATH: &'static str = "settings.toml";
pub const WWW: &'static str = "www";

lazy_static! {
	pub static ref SETTINGS: Settings =
		Settings::get().expect(&format!("Unable to parse {}", SETTINGS_PATH));
}

fn main() {
	// get cmd args
	let opt = Cli::from_args();

	// port setting
	match opt.port {
		Some(i) => env::set_var("ROCKET_PORT", format!("{}", i)),
		None => (),
	}

	// git repo update
	thread::spawn(|| loop {
		thread::sleep(time::Duration::from_secs(60));
		match SETTINGS.git.update() {
			Ok(s) => println!("git repo update: status = {}", s.status),
			Err(e) => eprintln!("{:?}", e),
		}
	});

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::index, routes::path,])
		.register(catchers![routes::not_found,])
		.launch();
}
