use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref COMMENT_REGEX: Regex = Regex::new("^\"").unwrap();
	static ref NEW_LINE_REGEX: Regex = Regex::new(r"^\r?\n").unwrap();
}

#[derive(Clone)]
pub struct Rule {
	pattern: Regex,
	ignore: bool
}

impl Rule {
	pub fn new(pattern: &str, ignore: bool) -> Self {
		Self {
			pattern: Regex::new(pattern).unwrap(),
			ignore
		}
	}
}

#[derive(Clone)]
pub struct Lexer {
	text: String,
	rules: Vec<Rule>,
	line: usize,
	column: usize,
	comment: bool,
	file_path: std::path::PathBuf,
	position: usize,
}

impl Lexer {
	pub fn new(text: String, rules: Vec<Rule>, file_path: std::path::PathBuf) -> Self {
		Self {
			text,
			rules,
			line: 1,
			column: 1,
			comment: false,
			file_path,
			position: 0,
		}
	}

	pub fn next(&mut self) -> Result<Option<(String, usize, usize, std::path::PathBuf)>, String> {
		let text = &self.text.as_str()[self.position..];
		if text.is_empty() {
			return Ok(None);
		}
		if text == "\"" {
			self.comment = !self.comment;
		}
		if self.comment {
			return Ok(None);
		}
		for rule in &self.rules {
			if let Some(captures) = rule.pattern.captures(text) {
				if let Some(capture) = captures.get(0) {
					let (command_line, command_column) = (self.line, self.column);
					let command = capture.as_str();
					let command_length = capture.end() - capture.start();
					self.position += command_length;
					if command.contains('\n') {
						self.line += command.matches('\n').count();
						self.column = command.split('\n').last().unwrap().len() + 1;
					} else {
						self.column += command_length;
					}
					if rule.ignore {
						return self.next();
					}
					return Ok(Some((command.to_string(), command_line, command_column, self.file_path.clone())));
				}
			}
		}
		Err(format!(
			"Syntax error at {}:{} in {:?} ({:?})",
			self.line,
			self.column,
			self.file_path.file_name().unwrap(),
			self.file_path.as_path()
		))
	}
}
