use std::{
	ffi::OsStr,
	fmt::Debug,
	fs::{metadata, read_to_string},
	os::unix::fs::PermissionsExt,
	path::Path,
	process::Command,
};

use anyhow::{Result, Context, anyhow};

// TODO could probably be
// done with 
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

	pub fn open<P>(path: P, skel: P) -> Result<File>
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
				.context("failed to convert extension to str")?
			{
				"md" => Self::html(md(&read_to_string(path)?, skel)?),
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

#[macro_export]
macro_rules! markdown {
	($e:expr) => {{
		use comrak::{markdown_to_html, ComrakOptions};
		let options = ComrakOptions {
			unsafe_: true,
			ext_table: true,
			ext_strikethrough: true,
			ext_tasklist: true,
			ext_superscript: true,
			..ComrakOptions::default()
			};
		markdown_to_html($e, &options)
		}};
}

pub fn md<P>(body: &str, skel: P) -> Result<String> 
	where
		P: AsRef<OsStr> + AsRef<Path> + Debug,
	{
	let skeleton: String = read_to_string(skel)?;

	Ok(skeleton.replace("{}", &markdown!(body)))
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
