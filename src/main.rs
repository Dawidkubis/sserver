#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
mod response;
mod cli;
mod routes;
mod rsp;

use cli::Cli;
use settings::Settings;

use std::{env, thread, time};
use std::process::Command;

use rocket::{catchers, routes};
use structopt::StructOpt;

pub const SETTINGS: &str = "settings.toml";

fn main() {
	// get cmd args
	let opt = Cli::from_args();

	// port setting
	if let Some(i) = opt.port {
		env::set_var("ROCKET_PORT", format!("{}", i));
	}

	// git repo update
	thread::spawn(|| loop {
		thread::sleep(time::Duration::from_secs(1));
		match Command::new("git").arg("pull").output() {
			Ok(s) => println!("git repo updated"),
			Err(e) => eprintln!("{:?}", e),
		}
	});

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::path,])
		.register(catchers![routes::not_found,])
		.launch();
}
