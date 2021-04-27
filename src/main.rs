mod interpreter;
use interpreter::Interpreter;

mod minifier;
use minifier::Minifier;

mod args;

use std::fs::File;


fn main() {
	let args: std::vec::Vec<String> = args::get_args();

	let mut help = args::HelpCommand::new();
	help.add_command("-r".into(), "[file]".into(), "Execute [file] with the interpreter".into());
	help.add_command("-m".into(), "[file] [out]".into(), "Minify [file] and write the result to [out]".into());
	// help.add_command("-c", "[file] [out]", "Transpile [file] into C, write the result to [out]");

	if args.len() > 0 && args[0] == "-help".to_string() {
		help.print();
		std::process::exit(0);
	}
	else if args.len() < 2 {
		println!("At least 2 parameters are required. Type -help to show all commands.");
		panic!();
	}

	let file = File::open(args[1].as_str()).unwrap();

	if args[0] == "-r".to_string() {
		let mut int = Interpreter::new(&file);

		match int.load() {
			Ok(_) => int.run(),

			Err(e) => {
				println!("File read error: {:?}", e);
				panic!();
			}
		}
	}
	else if args[0] == "-m".to_string() {
		if args.len() < 3 {
			panic!("Expected 3 arguments, got 2");
		}

		let outfile = File::create(args[2].as_str()).unwrap();
		let mut min = Minifier::new(&file, &outfile);

		match min.load() {
			Ok(_) => min.run(),

			Err(e) => {
				println!("File read error: {:?}", e);
				panic!();
			}
		}
	}
}
