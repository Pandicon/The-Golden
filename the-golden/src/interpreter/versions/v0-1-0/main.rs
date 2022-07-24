#[path = "./brackets_matcher.rs"] mod brackets_matcher;
use brackets_matcher::BracketsMatcher;

pub struct Runner {
	brackets_matcher: BracketsMatcher,
	raw_code: String
}

impl Runner {
	pub fn new(raw_code: String) -> Self {
		Self {
			brackets_matcher: BracketsMatcher::new(),
			raw_code
		}
	}

	pub fn run(&self) {
		println!("Running version 0.1.0");
		println!("Raw code: {}", self.raw_code);
	}
}