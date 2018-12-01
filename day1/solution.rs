use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	let mut file = File::open("input")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	println!("Here are the file contents:");
	//println!("{}", contents);

	let split = contents.split("\n");

	// let sum = 0;

	for s in split {
		println!("Line: {}", s);

		if s.starts_with("+") {
			println!("plus");
		} else if s.starts_with("-") {
			println!("minus");
		}
	}

	Ok(())
}