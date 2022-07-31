use std::io::Write;

pub struct Utils {}

impl Utils {
	pub fn ansi_escape_text(style: &str, text: &str, min_length: usize) -> String {
		let mut res = format!("\x1b[{}m{}", style, text);
		for _ in 0..min_length - text.len() {
			res += " ";
		}
		res += "\x1b[0m";
		res
	}

	pub fn get_input_line() -> String {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		input.trim().to_string()
	}

	pub fn next_char(s: &str) -> (char, &str) {
		match s.chars().next() {
			Some(c) => (c, s.split_at(c.len_utf8()).1),
			None => ('\0', s.split_at(0).1),
		}
	}

	pub fn flush_console() -> std::io::Result<()> {
		std::io::stdout().flush()
	}
}
