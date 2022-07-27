#[derive(Clone, Debug)]
pub struct Flags {
	pub action: Option<String>,
	pub code_path: Option<std::path::PathBuf>,
	pub debug: bool,
	pub debug_heavy: bool,
	pub raw_code_to_run: Option<String>,
	pub version: Option<String>
}

impl Flags {
	pub fn new() -> Self {
		Self {
			action: None,
			code_path: None,
			debug: false,
			debug_heavy: false,
			raw_code_to_run: None,
			version: None
		}
	}

	pub fn parse(&mut self, args: &[String]) {
		let args_count = args.len();
		let mut i = 0;
		while i < args_count {
			let argument = &args[i];
			let argument_lowercase = argument.to_lowercase();
			match argument_lowercase.as_str() {
				"--debug" => self.debug = true,
				"--debug-heavy" => {
					self.debug = true;
					self.debug_heavy = true;
				},
				"--version" => if self.version.is_none() && i + 1 < args_count {
					self.version = Some(args[i+1].clone());
				},
				"-" => if self.raw_code_to_run.is_none() && i + 1 < args_count {
					self.raw_code_to_run = Some(args[i+1].clone());
				},
				"run" => if self.action.is_none() {
					self.action = Some(String::from("run"));
					if self.code_path.is_none() && i + 1 < args_count && !args[i+1].starts_with('-') {
						let mut path = std::path::PathBuf::from(args[0].clone());
						path.pop();
						path.push(args[i+1].clone());
						path.set_file_name("maumivu.au");
						self.code_path = Some(path);
					}
				}
				_ => {}
			}
			i += 1;
		}
	}
}