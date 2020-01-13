use crate::{SETTINGS, WWW};
use anyhow::Result;
use rocket::request::Request;
use rocket::response::{self, content, NamedFile, Responder};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub enum File {
	Html(content::Html<String>),
	File(NamedFile),
}

impl File {
	pub fn open<P>(path: P) -> Result<File>
	where
		P: AsRef<OsStr> + AsRef<Path> + Debug,
	{
		let p = Path::new(&path);

		// TODO executable checking

		// extensions
		match p.extension() {
			Some(s) => match s
				.to_str()
				.ok_or(anyhow!("cannot convert filename to utf-8"))?
			{
				"md" => Ok(Self::Html(content::Html(md(&read_to_string(path)?)?))),
				_ => Ok(Self::File(NamedFile::open(path)?)),
			},
			None => Err(anyhow!("path {:?} has no extension", path)),
		}
	}
}

impl<'r> Responder<'r> for File {
	fn respond_to(self, r: &Request) -> response::Result<'r> {
		match self {
			Self::Html(c) => c.respond_to(r),
			Self::File(c) => c.respond_to(r),
		}
	}
}

#[macro_export]
macro_rules! markdown {
	($e:expr) => {{
		use comrak::{markdown_to_html, ComrakOptions};
		markdown_to_html($e, &ComrakOptions::default())
		}};
}

pub fn md(body: &str) -> Result<String> {
	let skeleton: String = read_to_string([WWW, &SETTINGS.skeleton].iter().collect::<PathBuf>())?;

	Ok(skeleton.replace("{}", &markdown!(body)))
}
