use std::collections::HashMap;

pub struct BracketsMatcher {
	pub brackets: HashMap<String, HashMap<usize, usize>>,
	brackets_mem: Vec<(String, usize, usize)>,
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
				("]".to_string(), "while".to_string()),
				("@]".to_string(), "do_while".to_string()),
				("']".to_string(), "while_local".to_string()),
				("'@]".to_string(), "do_while_local".to_string()),
			])
		}
	}

	pub fn match_brackets(&self, code: Vec<String>) {

	}

	fn num_equals(&self, left: String, right: String) -> i8 {
		if self.bracket_keys.get(&left) != self.bracket_keys.get(&right) {
			return 0;
		}
		if left == right {
			return 1;
		}
		return -1;
	}
}