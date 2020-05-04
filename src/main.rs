#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
mod response;
mod cli;
mod routes;
mod rsp;

use cli::Cli;

use std::process::Command;
use std::{env, thread, time};

use lazy_static::lazy_static;
use rocket::{catchers, routes};
use structopt::StructOpt;

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
			Ok(s) => if cfg!(debug_assertions) {println!("{:?}", s)} else {},
			Err(e) => eprintln!("{:?}", e),
		}
	});

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::path, routes::index])
		.register(catchers![routes::not_found,])
		.launch();
}
