/* #[macro_use] */
/* mod response; */
/* mod routes; */
/* mod rsp; */

use std::{
	process::Command,
	path::PathBuf,
	env,
	thread,
	time,
};

use structopt::StructOpt;
use actix_web::{get, web, App, HttpServer, Responder};

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	// handle command line arguments
	let cla = Cla::from_args();

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

	// http server init
	HttpServer::new(|| 
		App::new()
			.route("/{path}", web::get().to(index))
			)
        .bind(format!("127.0.0.1:{}", cla.port))?
        .run()
        .await
}

async fn index(path: web::Path<Option<PathBuf>>) -> impl Responder {
	"hmm"
}
