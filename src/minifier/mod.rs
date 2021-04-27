// Copyright (c) 2021 Thatguyjs All Rights Reserved.
// cmdlang file minifier

#[path = "../jump.rs"]
mod jump;
use jump::JumpAlias;

#[path = "../common.rs"]
mod common;
use common::*;

use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::vec::Vec;


pub struct Minifier<'a> {
	file: &'a File,
	source: Vec<u8>,

	outfile: &'a File,
	outstream: BufWriter<&'a File>,

	jumps: Vec<JumpAlias>
}


impl<'a> Minifier<'a> {

	pub fn new(file: &'a File, outfile: &'a File) -> Minifier<'a> {
		Minifier {
			file,
			source: vec![],

			outfile,
			outstream: BufWriter::new(outfile),

			jumps: vec![]
		}
	}


	pub fn load(&mut self) -> Result<(), std::io::Error> {
		load_file(self.file, &mut self.source)
	}


	pub fn run(&mut self) {
		let length = self.source.len();
		let mut ind: usize = 0;
		let mut ignore_space = true;

		while ind < length {
			let mut ch = self.source[ind] as char;

			if ignore_space && is_whitespace_char(ch) {
				ind += 1;
				continue;
			}

			match ch {

				'/' => {
					while ind < length - 1 && ch != '\n' {
						ch = self.source[ind] as char;
						ind += 1;
					}

					continue;
				}

				'{' => ignore_space = false,
				'}' => ignore_space = true,

				'!' | '@' => {
					let mut name_bytes: Vec<u8> = vec![];
					ind += 1;

					while ind < length && (self.source[ind] as char) != ';' {
						name_bytes.push(self.source[ind]);
						ind += 1;
					}

					let name = String::from_utf8_lossy(&name_bytes).to_string();
					let mut alias = self.jumps.len();
					let mut found = false;

					for jump in &self.jumps {
						if jump.name == name {
							found = true;
							alias = jump.alias;
							break;
						}
					}

					if !found {
						self.jumps.push(JumpAlias::new(name, alias));
					}

					self.outstream.write(&[ch as u8]);
					self.outstream.write(alias.to_string().as_bytes());
				}

				_ => ()

			}

			self.outstream.write(&[self.source[ind]]);

			ind += 1;
		}

		self.outstream.flush().unwrap();
	}

}
