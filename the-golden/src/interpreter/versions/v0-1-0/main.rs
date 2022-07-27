use std::collections::HashMap;

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
	brackets_matcher: BracketsMatcher,
	
	raw_code: String,
	rules: Vec<Regex>,
	code_path: std::path::PathBuf,

	brackets: HashMap<String, HashMap<usize, usize>>
}

impl Runner {
	pub fn new(raw_code: String, code_path: std::path::PathBuf) -> Self {
		let rules = vec![
			Regex::new(r"^!").unwrap(),
			Regex::new(r"^\[@?").unwrap(),
			Regex::new(r"^@?\]").unwrap(),
			Regex::new(r"^:\r?\n?").unwrap(),
			Regex::new("\"[^\"]*\"").unwrap()
		];
		Self {
			brackets_matcher: BracketsMatcher::new(),

			raw_code,
			rules,
			code_path,

			brackets: HashMap::new()
		}
	}

	pub fn run(&mut self) {
		println!("Running version 0.1.0");
		println!("Raw code: {}", self.raw_code);
		let lexer = Lexer::new(self.raw_code.clone(), self.rules.clone(), self.code_path.clone());
		let validator_result = Validator::run(lexer.clone());
		if let Err(e) = validator_result {
			println!("{}", e);
			return;
		}
		println!("Valid code!");
		let mut parser = Parser::new();
		let parser_result = parser.run(lexer);
		if let Err(e) = parser_result {
			println!("{}", e);
			return;
		}
		self.brackets_matcher.match_brackets(&parser.commands);
		self.brackets = self.brackets_matcher.brackets.clone();
		println!("{:?}", self.brackets_matcher.brackets);
	}
}