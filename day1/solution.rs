use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	let mut file = File::open("input")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	println!("Here are the file contents:");
	println!("{}", contents);
	Ok(())
}