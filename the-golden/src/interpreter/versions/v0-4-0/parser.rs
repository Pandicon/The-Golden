pub struct Parser {
	pub commands: Vec<String>,
	pub commands_info: Vec<(String, usize, usize, std::path::PathBuf)>,
}

impl Parser {
	pub fn new() -> Self {
		Self {
			commands: vec![],
			commands_info: vec![],
		}
	}

	pub fn run(&mut self, mut lexer: super::Lexer) -> Result<u8, String> {
		let mut t = lexer.next();
		while t.is_ok() && t.clone().unwrap().is_some() {
			let val = t.clone().unwrap().unwrap();
			let (command, ..) = val.clone();
			if !((command.starts_with('"') && command.ends_with('"')) || command.contains(':')) {
				self.commands.push(command);
				self.commands_info.push(val);
			}
			t = lexer.next();
		}
		if let Err(e) = t {
			return Err(e);
		}
		Ok(0)
	}
}
