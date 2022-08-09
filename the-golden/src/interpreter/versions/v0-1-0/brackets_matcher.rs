use std::collections::HashMap;

pub struct BracketsMatcher {
	pub brackets: HashMap<String, HashMap<usize, usize>>,
	brackets_mem: Vec<(String, usize, isize)>,
	bracket_keys: HashMap<String, String>,
	ending_brackets_keys: HashMap<String, String>
}

impl BracketsMatcher {
	pub fn new() -> Self {
		Self { 
			brackets: HashMap::from([
				("while".to_string(), HashMap::new()),
				("do_while".to_string(), HashMap::new()),
				("while_local".to_string(), HashMap::new()),
				("do_while_local".to_string(), HashMap::new())
			]),
			brackets_mem: vec![],
			bracket_keys: HashMap::from([
				("[".to_string(), "while".to_string()),
				("]".to_string(), "while".to_string()),
				("[@".to_string(), "do_while".to_string()),
				("@]".to_string(), "do_while".to_string()),
				("'[".to_string(), "while_local".to_string()),
				("']".to_string(), "while_local".to_string()),
				("'[@".to_string(), "do_while_local".to_string()),
				("'@]".to_string(), "do_while_local".to_string()),
			]),
			ending_brackets_keys: HashMap::from([
				("while".to_string(), "]".to_string()),
				("do_while".to_string(), "@]".to_string()),
				("while_local".to_string(), "']".to_string()),
				("do_while_local".to_string(), "'@]".to_string()),
			])
		}
	}

	pub fn match_brackets(&mut self, code: &[String]) {
		let starting_brackets = ["[", "[@", "'[", "'[@", ];
		let ending_brackets = ["]", "@]", "']", "'@]"];
		for (i, command) in code.iter().enumerate() {
			let command_str = command.as_str();
			if !starting_brackets.contains(&command_str) && !ending_brackets.contains(&command_str) {
				continue;
			}
			if starting_brackets.contains(&command_str) {
				self.brackets_mem.push((command.clone(), i, 0));
			}
			let mut keys_to_remove = vec![];
			for key in 0..self.brackets_mem.len() {
				self.brackets_mem[key].2 += self.num_equals(&self.brackets_mem[key].0, command);
				let wanted_end = self.ending_brackets_keys.get(self.bracket_keys.get(&self.brackets_mem[key].0).unwrap()).unwrap();
				if self.brackets_mem[key].2 == 0 && command == wanted_end {
					let category = self.bracket_keys.get(wanted_end).unwrap();
					let sub_map = self.brackets.get_mut(category).unwrap();
					sub_map.insert(self.brackets_mem[key].1, i);
					sub_map.insert(i, self.brackets_mem[key].1);
					keys_to_remove.push(key);
				}
			}
			for key in keys_to_remove {
				self.brackets_mem.remove(key);
			}
		}
	}

	fn num_equals(&self, left: &String, right: &String) -> isize {
		if self.bracket_keys.get(left) != self.bracket_keys.get(right) {
			return 0;
		}
		if left == right {
			return 1;
		}
		-1
	}
}