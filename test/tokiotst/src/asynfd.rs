
use std::error::Error;

pub async  fn async_read_file(fname :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	let contents = tokio::fs::read(fname).await?;
	Ok(contents)
}