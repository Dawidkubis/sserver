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
#[macro_use]
mod settings;
mod cli;
mod routes;
//mod rsp;

use cli::Cli;
use settings::Settings;
use std::{env, thread, time};
use structopt::StructOpt;

pub const SETTINGS_PATH: &str = "settings.toml";
pub const WWW: &str = "www";

lazy_static! {
	pub static ref SETTINGS: Settings = match Settings::get() {
		Ok(s) => s,
		Err(e) => panic!("Unable to parse {} : {}", SETTINGS_PATH, e),
	};
}

fn main() {
	// get cmd args
	let opt = Cli::from_args();

	// check settings
	&*SETTINGS;

	// port setting
	if let Some(i) = opt.port {
		env::set_var("ROCKET_PORT", format!("{}", i));
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
