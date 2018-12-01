use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
	let mut file = File::open("input")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	println!("Here are the file contents:");
	//println!("{}", contents);

	let split = contents.split("\n");

	let mut sum : i32 = 0;
	let mut sums = HashSet::new();

	sums.insert(sum);

	let mut reached_twice : i32 = 0;
	let mut found_reached_twice = false;

	while found_reached_twice == false {
		for s in split.clone() {
			println!("Line: {}", s);

			if s.starts_with("+") {
				println!("plus");
			} else if s.starts_with("-") {
				println!("minus");
			}

			// XXX: Hacky way of handling unparsed lines
			let parsed = s.parse::<i32>().unwrap_or(999999);
			if parsed != 999999 {
				sum += parsed;

				if !found_reached_twice {
					if sums.contains(&sum) {
						reached_twice = sum;
						found_reached_twice = true;
					}
				}

				sums.insert(sum);
			}
		}
	}

	println!("Sum: {}", sum);
	println!("Has reached twice: {}", found_reached_twice);
	println!("Reached twice first: {}", reached_twice);

	Ok(())
}