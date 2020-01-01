use rocket::response::content;
use std::fs::read_to_string;

pub type HTML = content::Html<String>;

#[macro_export]
macro_rules! markdown {
	($e:expr) => {{
		use comrak::{markdown_to_html, ComrakOptions};
		markdown_to_html($e, &ComrakOptions::default())
		}};
}

pub fn md(body: &str) -> String {
	let skeleton: String = read_to_string("frontend/main.html").unwrap();

	skeleton.replace("{}", &markdown!(body))
}
