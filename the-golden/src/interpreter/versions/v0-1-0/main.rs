use std::collections::HashMap;

use crate::Flags;
use rand::Rng;
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
	opposite_commands: HashMap<String, String>,

	brackets: HashMap<usize, usize>,
	raw_code: String,
	rules: Vec<Regex>,
	code_path: std::path::PathBuf,

	program_pointer: usize,

	loops: Vec<usize>,
	memory: [Vec<f64>; 2],
	memory_pointers: [usize; 2],
	active_memory: usize,

	input_cache: Option<String>,
}

impl Runner {
	pub fn new(raw_code: String, code_path: std::path::PathBuf, flags: Flags) -> Self {
		let rules = vec![
			Regex::new(r"^'?(\|-?[0-9]*\|)*!").unwrap(),   // increment
			Regex::new(r"^'?(\|-?[0-9]*\|)*~").unwrap(),   // decrement
			Regex::new(r"^'?(\|-?[0-9]*\|)*\+").unwrap(),  // add
			Regex::new(r"^'?(\|-?[0-9]*\|)*-").unwrap(),   // subtract
			Regex::new(r"^'?(\|-?[0-9]*\|)*\*").unwrap(),  // multiply
			Regex::new(r"^'?(\|-?[0-9]*\|)*/").unwrap(),   // divide
			Regex::new(r"^'?`").unwrap(),                  // generate a random number from 0 (inclusive) to 1 (exclusive)
			Regex::new(r"^'?(\|-?[0-9]*\|)*>").unwrap(),   // move right
			Regex::new(r"^'?(\|-?[0-9]*\|)*<").unwrap(),   // move left
			Regex::new(r"^'?_").unwrap(),                  // floor
			Regex::new(r"^'?&").unwrap(),                  // ceil
			Regex::new(r"^'?\^").unwrap(),                 // switch active memory
			Regex::new(r"^'?\[@?").unwrap(),               // (do-)while start
			Regex::new(r"^'?@?\]").unwrap(),               // (do-)while end
			Regex::new(r"^'?\$\.").unwrap(),               // input number
			Regex::new(r"^'?\$,").unwrap(),                // input character
			Regex::new(r"^'?\\\.").unwrap(),               // output number
			Regex::new(r"^'?\\,").unwrap(),                // output character
			Regex::new(r"^'?(\|-?[0-9]*\|)*\?=").unwrap(), // break if active memory address is equal to inactive memory address
			Regex::new(r"^'?(\|-?[0-9]*\|)*\?>").unwrap(), // break if active memory address is greater than inactive memory address
			Regex::new(r"^'?(\|-?[0-9]*\|)*\?<").unwrap(), // break if active memory address is less than inactive memory address
			Regex::new(r"^'?;").unwrap(),                  // swap main and local memory addresses
			Regex::new(r"^:\r?\n?").unwrap(),              // end of line
			Regex::new("^\"[^\"]*\"").unwrap(),            // comments
			Regex::new(r"^[ \t\f\v]").unwrap(),            // whitespace
		];
		Self {
			flags,

			brackets_matcher: BracketsMatcher::new(),

			brackets: HashMap::new(),
			brackets_categorised: HashMap::new(),
			opposite_commands: HashMap::from([
				("!".to_string(), "~".to_string()),
				("~".to_string(), "!".to_string()),
				("+".to_string(), "-".to_string()),
				("-".to_string(), "+".to_string()),
				("*".to_string(), "/".to_string()),
				("/".to_string(), "*".to_string()),
				(">".to_string(), "<".to_string()),
				("<".to_string(), ">".to_string()),
			]),

			raw_code,
			rules,
			code_path,

			program_pointer: 0,

			loops: vec![],
			memory: [vec![0.0], vec![0.0]],
			memory_pointers: [0, 0],
			active_memory: 0,

			input_cache: None,
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
			active_local_memory = match self.evaluate_command(command, &mut local_memory, &mut local_memory_pointers, active_local_memory) {
				Ok(val) => val,
				Err(e) => {
					println!("{}{}", Utils::ansi_escape_text("91", "ERROR", INFO_PREFIX_LENGTH), e);
					break;
				}
			};
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

	pub fn evaluate_command(&mut self, command: &str, local_memory: &mut [Vec<f64>; 2], local_memory_pointers: &mut [usize; 2], active_local_memory: usize) -> Result<usize, String> {
		let is_local = command.starts_with('\'');
		let raw_command = command;
		let command = if is_local { &command[1..] } else { command };
		let [(main_memory, main_memory_pointers, mut main_active_memory), (local_memory, local_memory_pointers, local_active_memory)] = if is_local {
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
		let split_command = command.split('|').collect::<Vec<&str>>();
		let (command, repeat) = if split_command.len() == 3 {
			let count_str = split_command[1];
			let num = if count_str.is_empty() {
				main_memory[main_active_memory][main_memory_pointers[main_active_memory]].floor() as i128
			} else if let Ok(val) = count_str.parse::<i128>() {
				val
			} else {
				1
			};
			let new_command = split_command[2];
			if num < 0 {
				if let Some(opposite_command) = self.opposite_commands.get(new_command) {
					(opposite_command.as_str(), -num)
				} else {
					(new_command, 0)
				}
			} else {
				(new_command, num)
			}
		} else {
			(command, 1)
		};
		for _ in 0..repeat {
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
				"*" => {
					main_memory[main_active_memory][main_memory_pointers[main_active_memory]] *=
						main_memory[(main_active_memory as isize - 1).abs() as usize][main_memory_pointers[(main_active_memory as isize - 1).abs() as usize]]
				}
				"/" => {
					main_memory[main_active_memory][main_memory_pointers[main_active_memory]] /=
						main_memory[(main_active_memory as isize - 1).abs() as usize][main_memory_pointers[(main_active_memory as isize - 1).abs() as usize]]
				}
				"`" => main_memory[main_active_memory][main_memory_pointers[main_active_memory]] = rand::thread_rng().gen(),
				">" => {
					main_memory_pointers[main_active_memory] += 1;
					if main_memory_pointers[main_active_memory] >= main_memory[main_active_memory].len() {
						main_memory[main_active_memory].push(0.0);
					}
				}
				"<" => {
					if main_memory_pointers[main_active_memory] == 0 {
						main_memory[main_active_memory].insert(0, 0.0);
						if !self.flags.disabled_warnings.too_left_pointer {
							println!("{}You moved to the -1 index in memory. This will not crash the program, but should generally be avoided (you can use the --disable-warnings flag to disable all warnings or --disable-too-left-pointer-warning to disable this particular warning)", Utils::ansi_escape_text("93", "WARNING", INFO_PREFIX_LENGTH));
						}
					} else {
						main_memory_pointers[main_active_memory] -= 1;
					}
				}
				"_" => main_memory[main_active_memory][main_memory_pointers[main_active_memory]] = main_memory[main_active_memory][main_memory_pointers[main_active_memory]].floor(),
				"&" => main_memory[main_active_memory][main_memory_pointers[main_active_memory]] = main_memory[main_active_memory][main_memory_pointers[main_active_memory]].ceil(),
				"^" => main_active_memory = (main_active_memory as isize - 1).abs() as usize,
				"$." => {
					if self.input_cache.is_none() {
						self.input_cache = Some(Utils::get_input_line());
					}
					let input = &self.input_cache.clone().unwrap();
					self.input_cache = None;
					main_memory[main_active_memory][main_memory_pointers[main_active_memory]] = match input.parse::<f64>() {
						Ok(val) => val,
						Err(e) => {
							return Err(format!("Failed to convert {} from input to a number: {}", input, e));
						}
					}
				}
				"$," => {
					if self.input_cache.is_none() {
						self.input_cache = Some(Utils::get_input_line());
					}
					let input = &self.input_cache.clone().unwrap();
					let (char, remainder) = Utils::next_char(input);
					self.input_cache = if !remainder.is_empty() { Some(remainder.to_string()) } else { None };
					main_memory[main_active_memory][main_memory_pointers[main_active_memory]] = (char as u32) as f64;
				}
				"\\." => {
					print!("{}", main_memory[main_active_memory][main_memory_pointers[main_active_memory]]);
					if let Err(e) = Utils::flush_console() {
						println!("{}{}", Utils::ansi_escape_text("91", "ERROR", INFO_PREFIX_LENGTH), e);
					}
				}
				"\\," => match char::from_u32(main_memory[main_active_memory][main_memory_pointers[main_active_memory]].floor() as u32) {
					Some(c) => {
						print!("{}", c);
						if let Err(e) = Utils::flush_console() {
							println!("{}{}", Utils::ansi_escape_text("91", "ERROR", INFO_PREFIX_LENGTH), e);
						}
					}
					None => {
						return Err(format!(
							"Failed to convert {} from memory to a character",
							main_memory[main_active_memory][main_memory_pointers[main_active_memory]].floor()
						));
					}
				},
				"[" => {
					if main_memory[main_active_memory][main_memory_pointers[main_active_memory]] == 0.0 {
						if let Some(index) = self.loops.iter().position(|value| *value == self.program_pointer) {
							self.loops.remove(index);
						}
						self.program_pointer = *self.brackets.get(&self.program_pointer).unwrap();
					} else if !self.loops.contains(&self.program_pointer) {
						self.loops.push(self.program_pointer);
					}
				}
				"]" | "@]" => {
					if main_memory[main_active_memory][main_memory_pointers[main_active_memory]] == 0.0 {
						if let Some(index) = self.loops.iter().position(|value| *value == self.program_pointer) {
							self.loops.remove(index);
						}
					} else {
						self.program_pointer = *self.brackets.get(&self.program_pointer).unwrap();
					}
				}
				"[@" => {
					if main_memory[main_active_memory][main_memory_pointers[main_active_memory]] == 0.0 && self.loops.contains(&self.program_pointer) {
						if let Some(index) = self.loops.iter().position(|value| *value == self.program_pointer) {
							self.loops.remove(index);
						}
						self.program_pointer = *self.brackets.get(&self.program_pointer).unwrap();
					} else if !self.loops.contains(&self.program_pointer) {
						self.loops.push(self.program_pointer);
					}
				}
				"?=" => {
					let inactive_memory = (main_active_memory as isize - 1).abs() as usize;
					if main_memory[main_active_memory][main_memory_pointers[main_active_memory]] == main_memory[inactive_memory][main_memory_pointers[inactive_memory]] {
						if let Some(current_loop) = self.loops.pop() {
							self.program_pointer = *self.brackets.get(&current_loop).unwrap();
						}
					}
				}
				"?>" => {
					let inactive_memory = (main_active_memory as isize - 1).abs() as usize;
					if main_memory[main_active_memory][main_memory_pointers[main_active_memory]] > main_memory[inactive_memory][main_memory_pointers[inactive_memory]] {
						if let Some(current_loop) = self.loops.pop() {
							self.program_pointer = *self.brackets.get(&current_loop).unwrap();
						}
					}
				}
				"?<" => {
					let inactive_memory = (main_active_memory as isize - 1).abs() as usize;
					if main_memory[main_active_memory][main_memory_pointers[main_active_memory]] < main_memory[inactive_memory][main_memory_pointers[inactive_memory]] {
						if let Some(current_loop) = self.loops.pop() {
							self.program_pointer = *self.brackets.get(&current_loop).unwrap();
						}
					}
				}
				";" => {
					std::mem::swap(
						&mut local_memory[local_active_memory][local_memory_pointers[local_active_memory]],
						&mut main_memory[main_active_memory][main_memory_pointers[main_active_memory]],
					);
				}
				_ => {}
			}
		}
		self.program_pointer += 1;
		self.active_memory = if is_local { local_active_memory } else { main_active_memory };
		if self.flags.debug_heavy {
			println!("\n{}Raw command: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH), raw_command);
			println!("{}Command executed: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH), command);
			println!(
				"{}Command was executed on local memory: {:?}",
				Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH),
				is_local
			);
			println!("{}Command repetitions: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH), repeat);
			println!("{}Global memory: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH), self.memory);
			println!("{}Global memory pointers: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH), self.memory_pointers);
			println!("{}Active global memory: {:?}", Utils::ansi_escape_text("34", "HEAVY DEBUG", INFO_PREFIX_LENGTH), self.active_memory);
			std::thread::sleep(std::time::Duration::from_millis(500));
		}
		Ok(if is_local { main_active_memory } else { local_active_memory })
	}
}
