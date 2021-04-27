// Include multiple files in a program

use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;


pub fn load_file(mut file: &File, dest: &mut Vec<u8>) -> Result<(), std::io::Error> {
	let mut data = String::new();
	let mut result = String::new();

	match file.read_to_string(&mut data) {
		Ok(_) => {
			for line in data.lines() {
				let comment = line.find('/');
				let include = line.find('|');

				if include == None || (comment != None && comment.unwrap() < include.unwrap()) {
					result.push_str(line);
					result.push('\n');
					continue;
				}

				let mut path = &line[(include.unwrap() + 1)..];
				path = &path[..path.find('|').unwrap()];

				let include_file = File::open(path).unwrap();
				load_file(&include_file, dest).unwrap();
			}

			dest.extend_from_slice(result.as_bytes());
			Ok(())
		}

		Err(e) => Err(e)
	}
}
