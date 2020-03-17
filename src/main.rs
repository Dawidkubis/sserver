#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
mod response;
mod cli;
mod routes;
mod rsp;

use cli::Cli;

use std::{env, thread, time};
use std::process::Command;

use rocket::{catchers, routes};
use structopt::StructOpt;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref OPT: Cli = Cli::from_args();
}

fn main() {
	// port setting
	env::set_var("ROCKET_PORT", format!("{}", OPT.port));

	// git repo update
	thread::spawn(|| loop {
		thread::sleep(time::Duration::from_secs(1));
		match Command::new("git").arg("pull").output() {
			Ok(s) => println!("{:?}", s),
			Err(e) => eprintln!("{:?}", e),
		}
	});

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::path,])
		.register(catchers![routes::not_found,])
		.launch();
}
