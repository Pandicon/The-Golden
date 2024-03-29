use regex::Regex;

use super::INFO_PREFIX_LENGTH;
use crate::Utils;

pub struct Validator {}

impl Validator {
	pub fn run(mut lexer: super::Lexer, heavy_debug: bool, ansi_enabled: bool) -> Result<u8, String> {
		let mut t = lexer.next();
		if heavy_debug {
			println!("{}Matched command: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH, ansi_enabled), t);
		}
		let mut p = t.clone();
		while t.is_ok() && t.clone().unwrap().is_some() {
			p = t;
			t = lexer.next();
			if heavy_debug {
				println!("{}Matched command: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH, ansi_enabled), t);
			}
		}
		if let Err(e) = t {
			return Err(e);
		}
		let (command, line, column, file_path) = p.unwrap().unwrap();
		if !Regex::new(r":\n?\r?").unwrap().is_match(&command) {
			return Err(format!(
				"Syntax error at {}:{} in {:?} ({:?}) - ':' expected",
				line,
				column,
				file_path.file_name().unwrap(),
				file_path.as_path()
			));
		}
		Ok(0)
	}
}
