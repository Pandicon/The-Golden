use crate::Utils;

#[derive(Clone, Debug)]
pub struct Warnings {
	pub too_left_pointer: bool,
}

#[derive(Clone, Debug)]
pub struct Flags {
	pub disabled_warnings: Warnings,

	pub action: Option<String>,
	pub code_path: Option<std::path::PathBuf>,
	pub debug: bool,
	pub debug_heavy: bool,
	pub no_brainfuck: bool,
	pub no_console: bool,
	pub raw_code_to_run: Option<String>,
	pub sebek: [Option<f64>; 3],
	pub version: Option<String>,
}

impl Default for Flags {
	fn default() -> Self {
		Self::new()
	}
}

impl Flags {
	pub fn new() -> Self {
		Self {
			disabled_warnings: Warnings { too_left_pointer: false },

			action: None,
			code_path: None,
			debug: false,
			debug_heavy: false,
			no_brainfuck: false,
			no_console: false,
			raw_code_to_run: None,
			sebek: [None, None, None],
			version: None,
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
				}
				"--hide-console" => self.no_console = true,
				"--version" => {
					if self.version.is_none() && i + 1 < args_count {
						self.version = Some(args[i + 1].clone());
					}
				}
				"--disable-warnings" => self.disabled_warnings = Warnings { too_left_pointer: true },
				"--disable-too-left-pointer-warning" => self.disabled_warnings.too_left_pointer = true,
				"--no-brainfuck" => self.no_brainfuck = true,
				"--sebek" => {
					if i + 1 < args_count {
						self.sebek = Utils::parse_sebek(&args[i + 1]);
					}
				}
				"-" => {
					if self.raw_code_to_run.is_none() && i + 1 < args_count {
						self.raw_code_to_run = Some(args[i + 1].clone());
					}
				}
				"run" => {
					if self.action.is_none() {
						self.action = Some(String::from("run"));
						if self.code_path.is_none() && i + 1 < args_count && !args[i + 1].starts_with('-') {
							let mut path = if args[i + 1] != "." && (args[i + 1].starts_with('.') || args[i + 1].starts_with('/')) {
								let mut p = std::path::PathBuf::from(args[0].clone());
								p.pop();
								p.push(args[i + 1].clone());
								p
							} else {
								std::path::PathBuf::from(args[i + 1].clone())
							};
							if path.is_file() {
								path.set_file_name("maumivu.au");
							} else {
								path.push("maumivu.au");
							}
							self.code_path = Some(path);
						}
					}
				}
				_ => {}
			}
			i += 1;
		}
	}
}
