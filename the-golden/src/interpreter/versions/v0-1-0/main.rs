use std::collections::HashMap;

use crate::Flags;
use regex::Regex;

#[path = "./brackets_matcher.rs"]
mod brackets_matcher;
use brackets_matcher::BracketsMatcher;
#[path = "./lexer.rs"]
mod lexer;
pub use lexer::Lexer;
#[path = "./parser.rs"]
mod parser;
use crate::Utils;
pub use parser::Parser;
#[path = "./validator.rs"]
mod validator;
use validator::Validator;

const INFO_PREFIX_LENGTH: usize = 12;

pub struct Runner {
	flags: Flags,

	brackets_matcher: BracketsMatcher,
	brackets_categorised: HashMap<String, HashMap<usize, usize>>,

	brackets: HashMap<usize, usize>,
	raw_code: String,
	rules: Vec<Regex>,
	code_path: std::path::PathBuf,

	program_pointer: usize,

	loops: Vec<usize>,
	memory: [Vec<f64>; 2],
	memory_pointers: [usize; 2],
	active_memory: usize,
}

impl Runner {
	pub fn new(raw_code: String, code_path: std::path::PathBuf, flags: Flags) -> Self {
		let rules = vec![
			Regex::new(r"^'?!").unwrap(),
			Regex::new(r"^'?\[@?").unwrap(),
			Regex::new(r"^'?@?\]").unwrap(),
			Regex::new(r"^:\r?\n?").unwrap(),
			Regex::new("\"[^\"]*\"").unwrap(),
		];
		Self {
			flags,

			brackets_matcher: BracketsMatcher::new(),

			brackets: HashMap::new(),
			brackets_categorised: HashMap::new(),
			raw_code,
			rules,
			code_path,

			program_pointer: 0,

			loops: vec![],
			memory: [vec![0.0], vec![0.0]],
			memory_pointers: [0, 0],
			active_memory: 0,
		}
	}

	pub fn run(&mut self) {
		if self.flags.debug {
			println!("{}Running version 0.1.0", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH));
			println!("{}Raw code: {}", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH), self.raw_code);
		}
		let lexer = Lexer::new(self.raw_code.clone(), self.rules.clone(), self.code_path.clone());
		let validator_result = Validator::run(lexer.clone(), self.flags.debug_heavy);
		if let Err(e) = validator_result {
			println!("{}{}", Utils::ansi_escape_text("91", "ERROR", INFO_PREFIX_LENGTH), e);
			return;
		}
		if self.flags.debug {
			println!("{}Valid code!", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH));
		}
		let mut parser = Parser::new();
		let parser_result = parser.run(lexer);
		if let Err(e) = parser_result {
			println!("{}{}", Utils::ansi_escape_text("91", "ERROR", INFO_PREFIX_LENGTH), e);
			return;
		}
		if self.flags.debug {
			println!("{}Parsed commands: {:?}", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH), parser.commands);
		}
		self.brackets_matcher.match_brackets(&parser.commands);
		self.brackets_categorised = self.brackets_matcher.brackets.clone();
		if self.flags.debug_heavy {
			println!(
				"{}Matched brackets: {:?}",
				Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH),
				self.brackets_matcher.brackets
			);
		}
		for loop_type in self.brackets_categorised.keys() {
			let map = self.brackets_categorised.get(loop_type).unwrap();
			for (key, value) in map.iter() {
				self.brackets.insert(*key, *value);
			}
		}
		if self.flags.debug_heavy {
			println!(
				"{}Matched brackets uncategorised: {:?}",
				Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH),
				self.brackets
			);
		}
		if self.flags.debug {
			println!("{}----- START OF CODE EXECUTION -----", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH));
		}
		let mut local_memory: [Vec<f64>; 2] = [vec![0.0], vec![0.0]];
		let mut local_memory_pointers: [usize; 2] = [0, 0];
		let mut active_local_memory: usize = 0;
		let program_length = parser.commands.len();
		while self.program_pointer < program_length {
			let command = &parser.commands[self.program_pointer];
			active_local_memory = self.evaluate_command(command, &mut local_memory, &mut local_memory_pointers, active_local_memory);
		}
		if self.flags.debug {
			println!("\n{}----- END OF CODE EXECUTION -----", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH));
		}
		if self.flags.debug {
			println!("{}Main memory:", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH));
			println!("{}{:?}", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH), self.memory);
			println!("{}Local memory:", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH));
			println!("{}{:?}", Utils::ansi_escape_text("94", "DEBUG", INFO_PREFIX_LENGTH), local_memory);
		}
	}

	pub fn evaluate_command(&mut self, command: &str, local_memory: &mut [Vec<f64>; 2], local_memory_pointers: &mut [usize; 2], active_local_memory: usize) -> usize {
		let is_local = command.starts_with('\'');
		let command = if is_local { &command[1..] } else { command };
		let [(main_memory, main_memory_pointers, main_active_memory), (local_memory, local_memory_pointers, active_local_memory)] = if is_local {
			[
				(local_memory, local_memory_pointers, active_local_memory),
				(&mut self.memory, &mut self.memory_pointers, self.active_memory),
			]
		} else {
			[
				(&mut self.memory, &mut self.memory_pointers, self.active_memory),
				(local_memory, local_memory_pointers, active_local_memory),
			]
		};
		match command {
			"!" => main_memory[main_active_memory][main_memory_pointers[main_active_memory]] += 1.0,
			"~" => main_memory[main_active_memory][main_memory_pointers[main_active_memory]] -= 1.0,
			"+" => {
				main_memory[main_active_memory][main_memory_pointers[main_active_memory]] +=
					main_memory[(main_active_memory as isize - 1).abs() as usize][main_memory_pointers[(main_active_memory as isize - 1).abs() as usize]]
			}
			"-" => {
				main_memory[main_active_memory][main_memory_pointers[main_active_memory]] -=
					main_memory[(main_active_memory as isize - 1).abs() as usize][main_memory_pointers[(main_active_memory as isize - 1).abs() as usize]]
			}
			_ => {}
		}
		self.program_pointer += 1;
		self.active_memory = if is_local { active_local_memory } else { main_active_memory };
		if is_local {
			main_active_memory
		} else {
			active_local_memory
		}
	}
}
