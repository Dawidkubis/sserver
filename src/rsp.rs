use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rsp {
	pub uri: String,
	pub file: String,
}
