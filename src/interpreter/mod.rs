// Copyright (c) 2021 Thatguyjs All Rights Reserved.
// cmdlang file interpreter

#[path = "../jump.rs"]
mod jump;
use jump::Jump;

#[path = "../common.rs"]
mod common;
use common::*;

use std::fs::File;
use std::vec::Vec;


pub struct Interpreter<'a> {
	file: &'a File,
	source: Vec<u8>,

	cells: [u16; 30000],
	pointer: usize, // Cell pointer

	jumps: Vec<Jump>
}


impl<'a> Interpreter<'a> {

	pub fn new(file: &'a File) -> Interpreter {
		Interpreter {
			file,
			source: vec![],

			cells: [0; 30000],
			pointer: 0,

			jumps: vec![]
		}
	}


	pub fn load(&mut self) -> Result<(), std::io::Error> {
		load_file(self.file, &mut self.source)
	}


	pub fn run(&mut self) {
		let length = self.source.len();
		let mut ind: usize = 0;

		while ind < length {
			let mut ch = self.source[ind] as char;

			if is_whitespace_char(ch) {
				ind += 1;
				continue;
			}

			match ch {

				'/' => {
					while ind < length - 1 && ch != '\n' {
						ind += 1;
						ch = self.source[ind] as char;
					}
				}


				'+' => {
					if self.cells[self.pointer] == u16::MAX {
						self.cells[self.pointer] = 0;
					}
					else {
						self.cells[self.pointer] += 1;
					}
				}

				'-' => {
					if self.cells[self.pointer] == 0 {
						self.cells[self.pointer] = u16::MAX;
					}
					else {
						self.cells[self.pointer] -= 1;
					}
				}


				'>' => {
					self.pointer += 1;
				}

				'<' => {
					self.pointer -= 1;
				}


				'[' => {
					if self.cells[self.pointer] == 0 {
						let mut depth: usize = 0;

						ind += 1;
						ch = self.source[ind] as char;

						while ind < length - 1 && (ch != ']' || depth > 0) {
							if ch == '[' { depth += 1; }
							else if ch == ']' { depth -= 1; }

							ind += 1;
							ch = self.source[ind] as char;
						}
					}
				}

				']' => {
					if self.cells[self.pointer] != 0 {
						let mut depth: usize = 0;

						ind -= 1;
						ch = self.source[ind] as char;

						while ind > 0 && (ch != '[' || depth > 0) {
							if ch == ']' { depth += 1; }
							else if ch == '[' { depth -= 1; }

							ind -= 1;
							ch = self.source[ind] as char;
						}
					}
				}


				'!' => {
					let mut name_bytes: Vec::<u8> = vec![];
					ind += 1;

					while ind < length && (self.source[ind] as char) != ';' {
						name_bytes.push(self.source[ind]);
						ind += 1;
					}

					let name = String::from_utf8_lossy(&name_bytes).to_string();
					let mut found = false;

					for jump in &self.jumps {
						if jump.name == name {
							found = true;
							break;
						}
					}

					if !found {
						self.jumps.push(Jump::new(ind, name));
					}
				}

				'@' => {
					let mut name_bytes: Vec::<u8> = vec![];
					ind += 1;

					while ind < length && (self.source[ind] as char) != ';' {
						name_bytes.push(self.source[ind]);
						ind += 1;
					}

					let name = String::from_utf8_lossy(&name_bytes);

					for jump in &self.jumps {
						if jump.name == name {
							ind = jump.index;
							break;
						}
					}
				}


				'=' => {
					let mut num_bytes: Vec::<u8> = vec![];
					ind += 1;

					while ind < length && (self.source[ind] as char) != '(' {
						if is_whitespace_char(self.source[ind] as char) {
							ind += 1;
							continue;
						}

						num_bytes.push(self.source[ind]);
						ind += 1;
					}

					let num = String::from_utf8_lossy(&num_bytes).parse::<u16>().unwrap();

					if self.cells[self.pointer] != num {
						ind += 1;
						ch = self.source[ind] as char;

						let mut depth: usize = 0;

						while ind < length - 1 && (ch != ')' || depth > 0) {
							if ch == '(' { depth += 1; }
							else if ch == ')' { depth -= 1; }

							ind += 1;
							ch = self.source[ind] as char;
						}
					}
				}

				')' => {}


				',' => {
					// Todo: Input a character
				}

				'.' => {
					print!("{}", String::from_utf16_lossy(&[self.cells[self.pointer]]));
				}

				':' => {
					print!("{}", self.cells[self.pointer] as u16);
				}


				'{' => {
					ind += 1;

					let mut str_bytes: Vec::<u8> = vec![];

					while ind < length && (self.source[ind] as char) != '}' {
						str_bytes.push(self.source[ind]);
						ind += 1;
					}

					print!("{}", String::from_utf8_lossy(&str_bytes));
				}


				other => println!("Unknown command: {}", other)

			}

			ind += 1;
		}
	}

}
