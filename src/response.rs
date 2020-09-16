use crate::OPT;

use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::{metadata, read_to_string};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Result};
use rocket::request::Request;
use rocket::response::{self, content, NamedFile, Responder};
use comrak::{markdown_to_html, ComrakOptions};

pub enum File {
	Html(String),
	File(NamedFile),
}

impl File {
	fn html(s: String) -> Result<Self> {
		Ok(Self::Html(s))
	}

	fn file(p: impl AsRef<Path>) -> Result<Self> {
		Ok(Self::File(NamedFile::open(p)?))
	}

	pub fn open<P>(path: P) -> Result<File>
	where
		P: AsRef<OsStr> + AsRef<Path> + Debug,
	{
		let p = Path::new(&path);

		if is_exec(&p) {
			return Self::html(source(&p)?);
		}

		// extensions
		match p.extension() {
			Some(s) => match s
				.to_str()
				.ok_or_else(|| anyhow!("cannot convert filename to utf-8"))?
			{
				"md" => Self::html(md(&read_to_string(path)?)?),
				_ => Self::file(path),
			},
			None => Err(anyhow!("path {:?} has no extension", path)),
		}
	}
}

impl<'r> Responder<'r> for File {
	fn respond_to(self, r: &Request) -> response::Result<'r> {
		match self {
			Self::Html(c) => content::Html(c).respond_to(r),
			Self::File(c) => c.respond_to(r),
		}
	}
}

fn markdown(e: &str) -> String {
//		ext_table: true,
//		ext_strikethrough: true,
//		ext_tasklist: true,
	let mut options = ComrakOptions::default();
	options.render.unsafe_ = true;
	options.extension.superscript = true;
	markdown_to_html(e, &options)
}

pub fn md(body: &str) -> Result<String> {
	let skeleton: String = read_to_string(&OPT.skel)?;

	Ok(skeleton.replace("{}", &markdown(body)))
}

fn is_exec(path: impl AsRef<Path>) -> bool {
	match metadata(path) {
		Ok(s) => s.permissions().mode() & 0o111 != 0,
		Err(_) => false,
	}
}

fn source(path: &Path) -> Result<String> {
	let r = Command::new(path).output()?.stdout;
	Ok(String::from_utf8(r)?)
}
