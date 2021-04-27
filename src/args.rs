// Argument helper functions

use std::env;
use std::vec;


pub fn get_args() -> vec::Vec<String> {
	let mut args: vec::Vec<String> = vec![];

	for arg in env::args() {
		if !arg.ends_with(".exe") {
			args.push(arg);
		}
	}

	return args;
}


pub struct HelpCommand {
	commands: vec::Vec<(String, String, String)>
}


impl HelpCommand {

	pub fn new() -> HelpCommand {
		HelpCommand {
			commands: vec![]
		}
	}


	pub fn add_command(&mut self, name: String, args: String, desc: String) {
		self.commands.push((name, args, desc));
	}


	pub fn print(&self) {
		for command in self.commands.iter() {
			println!("{} {}\t\t{}", command.0, command.1, command.2);
		}
	}

}
