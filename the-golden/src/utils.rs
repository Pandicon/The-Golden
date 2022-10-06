use std::io::Write;

pub struct Utils {}

impl Utils {
	pub fn ansi_escape_text(style: &str, text: &str, min_length: usize, enabled_ansi: bool) -> String {
		let mut res = format!("{}{}", if enabled_ansi { format!("\x1b[{}m", style) } else { String::new() }, text);
		for _ in 0..min_length - text.len() {
			res += " ";
		}
		if enabled_ansi {
			res += "\x1b[0m";
		}
		res
	}

	pub fn get_input_line() -> String {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		input
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

	pub fn parse_sebek(input: &str) -> [Option<f64>; 3] {
		let mut sebek = [None, None, None];
		let args_vec = input.split('|').filter_map(|val| val.parse::<f64>().ok()).collect::<Vec<f64>>();
		for (i, &val) in args_vec.iter().enumerate() {
			if i >= sebek.len() {
				break;
			}
			sebek[i] = Some(val);
		}
		sebek
	}

	pub fn numeric_part_end(input: &str) -> usize {
		let mut i = 0;
		let mut period = false;
		for char in input.chars() {
			match char {
				'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {}
				'.' => {
					if period {
						return i;
					}
					period = true;
				}
				_ => return i,
			}
			i += 1;
		}
		i
	}
}
