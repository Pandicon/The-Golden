use regex::Regex;

#[path = "./brackets_matcher.rs"] mod brackets_matcher;
use brackets_matcher::BracketsMatcher;
#[path = "./lexer.rs"] mod lexer;
pub use lexer::Lexer;
#[path = "./validator.rs"] mod validator;
use validator::Validator;

pub struct Runner {
	brackets_matcher: BracketsMatcher,
	
	raw_code: String,
	rules: Vec<Regex>,
	code_path: std::path::PathBuf
}

impl Runner {
	pub fn new(raw_code: String, code_path: std::path::PathBuf) -> Self {
		let rules = vec![
			Regex::new(r"^!").unwrap(),
			Regex::new(r"^:\r?\n?").unwrap(),
			Regex::new("\"[^\"]*\"").unwrap()
		];
		Self {
			brackets_matcher: BracketsMatcher::new(),
			raw_code,
			rules,
			code_path
		}
	}

	pub fn run(&self) {
		println!("Running version 0.1.0");
		println!("Raw code: {}", self.raw_code);
		let validator_result = Validator::run(Lexer::new(self.raw_code.clone(), self.rules.clone(), self.code_path.clone()));
		match validator_result {
			Err(e) => {
				println!("{}", e);
				return;
			},
			_ => {}
		}
		println!("Valid code!");
	}
}