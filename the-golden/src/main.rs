use dotenv::dotenv;
use std::env;

#[path = "./interpreter/interpreter.rs"] mod interpreter;
use interpreter::Interpreter;

fn main() {
	dotenv().ok();
	if env::var("RUST_LOG").is_err() {
		env::set_var("RUST_LOG", "INFO");
	}
	match env::var("LOGS") {
		Ok(val) => {
			if val.to_lowercase() != "on" && cfg!(target_os = "windows") {
				winconsole::window::hide();
			}
		}
		Err(_) => {
			if cfg!(target_os = "windows") {
				winconsole::window::hide();
			}
		}
	}
	tracing_subscriber::fmt::init();

	let args: Vec<String> = std::env::args().collect();
	let actions = [String::from("run")];
	let mut action = String::new();
	let mut version = String::from("latest");
	for arg in &args {
		if arg.starts_with("--version:") {
			version = arg.split_once("--version:").unwrap().1.to_string();
		} else if actions.contains(&arg.to_lowercase()) {
			action = arg.to_lowercase();
		}
	}
	if action == *"run" {
		let mut path = std::path::PathBuf::from(args[0].clone());
		path.pop();
		if args.len() > 2 {
			path.push(args[2].clone());
		}
		path.set_file_name("maumivu.au");
		let code = match std::fs::read_to_string(&path) {
			Ok(c) => c,
			Err(e) => panic!("{}", e)
		};
		Interpreter::new(version, code, path).run();
	} else {
		todo!()
	}
}
