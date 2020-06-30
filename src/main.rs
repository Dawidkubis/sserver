#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
mod response;
use response::File;
mod settings;
use settings::Settings;

use std::{
	process::Command,
	path::{PathBuf, Path},
	env,
	thread,
	time,
};

use rocket::{routes, get, State};
use structopt::StructOpt;
use anyhow::Result;

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

/// index of the site
#[get("/")]
pub fn index(rsp: State<Settings>, cla: State<Cla>) -> Result<File> {

	File::open(rsp.index)
}

/// any other path
#[get("/<path..>")]
pub fn path(rsp: State<Settings>, cla: State<Cla>, path: PathBuf) -> Result<File> {
	if let Some(s) = rsp.get {
		for i in s.into_iter() {
			if path.as_path() == Path::new(&i.uri) {
				return File::open(i.file);
			}
		}
	}

	File::open(path)
}

fn main() {
	let cla = Cla::from_args();
	let set = Settings::get(cla.settings)?;
	
	// port setting
	env::set_var("ROCKET_PORT", format!("{}", cla.port));

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
		.manage(cla)
		.manage(set)
		.mount("/", routes![path, index])
		.launch();
}
