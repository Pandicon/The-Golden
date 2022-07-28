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
}