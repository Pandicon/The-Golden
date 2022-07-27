use std::collections::HashMap;

use crate::Flags;
use regex::Regex;

#[path = "./brackets_matcher.rs"] mod brackets_matcher;
use brackets_matcher::BracketsMatcher;
#[path = "./lexer.rs"] mod lexer;
pub use lexer::Lexer;
#[path = "./parser.rs"] mod parser;
pub use parser::Parser;
#[path = "./validator.rs"] mod validator;
use validator::Validator;

pub struct Runner {
	flags: Flags,

	brackets_matcher: BracketsMatcher,
	
	raw_code: String,
	rules: Vec<Regex>,
	code_path: std::path::PathBuf,

	brackets: HashMap<String, HashMap<usize, usize>>
}

impl Runner {
	pub fn new(raw_code: String, code_path: std::path::PathBuf, flags: Flags) -> Self {
		let rules = vec![
			Regex::new(r"^!").unwrap(),
			Regex::new(r"^\[@?").unwrap(),
			Regex::new(r"^@?\]").unwrap(),
			Regex::new(r"^:\r?\n?").unwrap(),
			Regex::new("\"[^\"]*\"").unwrap()
		];
		Self {
			flags,

			brackets_matcher: BracketsMatcher::new(),

			raw_code,
			rules,
			code_path,

			brackets: HashMap::new()
		}
	}

	pub fn run(&mut self) {
		if self.flags.debug {
			println!("Running version 0.1.0");
			println!("Raw code: {}", self.raw_code);
		}
		let lexer = Lexer::new(self.raw_code.clone(), self.rules.clone(), self.code_path.clone());
		let validator_result = Validator::run(lexer.clone(), self.flags.debug_heavy);
		if let Err(e) = validator_result {
			println!("{}", e);
			return;
		}
		if self.flags.debug {
			println!("Valid code!");
		}
		let mut parser = Parser::new();
		let parser_result = parser.run(lexer);
		if let Err(e) = parser_result {
			println!("{}", e);
			return;
		}
		if self.flags.debug {
			println!("Parsed commands: {:?}", parser.commands);
		}
		self.brackets_matcher.match_brackets(&parser.commands);
		self.brackets = self.brackets_matcher.brackets.clone();
		if self.flags.debug_heavy {
			println!("Matched brackets: {:?}", self.brackets_matcher.brackets);
		}

		if self.flags.debug {
			println!("----- START OF CODE EXECUTION -----");
		}
	}
}