#[path = "./v0-1-0/main.rs"] mod v0_1_0;

pub struct Handler {
	versions: [String; 1]
}

impl Handler {
	pub fn new() -> Self {
		Self { versions: [String::from("0.1.0")] }
	}

	pub fn parse_version(&self, mut version: String) -> String {
		version = version.to_lowercase();
		if version == *"latest" || !self.versions.contains(&version) {
			version = self.versions.last().unwrap().to_string();
		}
		version
	}

	pub fn run(&self, version: String, code: String, code_path: std::path::PathBuf) {
		match version.as_str() {
			"0.1.0" => v0_1_0::Runner::new(code, code_path).run(),
			_ => panic!("Couldn't launch version {}", &version)
		}
	}
}