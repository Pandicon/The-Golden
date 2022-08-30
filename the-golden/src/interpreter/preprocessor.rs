use regex::Regex;

use crate::Utils;

#[derive(Clone, Debug)]
pub struct Warnings {
	pub too_left_pointer: bool,
}

#[derive(Clone, Debug)]
pub struct Preprocessor {
	pub disabled_warnings: Warnings,

	pub no_console: bool,
	pub sebek: [Option<f64>; 3],
	pub version: Option<String>,
}

impl Preprocessor {
	pub fn new() -> Self {
		Self {
			disabled_warnings: Warnings { too_left_pointer: false },

			no_console: false,
			sebek: [None, None, None],
			version: None,
		}
	}

	pub fn run(&mut self, code: &str) {
		let rule = Regex::new(crate::PREPROCESSOR_REGEX).unwrap();
		let statements = rule.find_iter(code).map(|m| m.as_str().trim()).collect::<Vec<&str>>();
		for &statement in &statements {
			let mut statement_chars = statement.chars();
			statement_chars.next();
			if statement.ends_with(':') {
				statement_chars.next_back();
			}
			let args = statement_chars.as_str().split(' ').collect::<Vec<&str>>();
			if args.is_empty() {
				continue;
			}
			let args_count = args.len();
			match args[0].to_lowercase().as_str() {
				"version" => {
					if args_count < 2 {
						continue;
					}
					self.version = Some(args[1].to_string());
				}
				"noconsole" | "no-console" | "no_console" => {
					if args_count < 2 {
						self.no_console = true;
						continue;
					}
					self.no_console = args[1].to_lowercase() != "false";
				}
				"disablewarnings" | "disable-warnings" | "disable_warnings" => {
					if args_count < 2 {
						continue;
					}
					match args[1].to_lowercase().as_str() {
						"too-left-pointer" | "tooleftpointer" => {
							self.disabled_warnings.too_left_pointer = true;
						}
						_ => {}
					}
				}
				"sebek" => {
					if args_count < 2 {
						continue;
					}
					self.sebek = Utils::parse_sebek(args[1]);
				}
				_ => {}
			}
		}
	}
}
