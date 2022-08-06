use crate::Flags;

#[path = "./v0-1-0/main.rs"]
mod v0_1_0;
#[path = "./v0-2-0/main.rs"]
mod v0_2_0;

pub struct Handler {
	versions: [String; 2],
}

impl Handler {
	pub fn new() -> Self {
		Self {
			versions: [String::from("0.1.0"), String::from("0.2.0")],
		}
	}

	pub fn parse_version(&self, mut version: String) -> String {
		version = version.to_lowercase();
		if version == *"latest" || !self.versions.contains(&version) {
			version = self.versions.last().unwrap().to_string();
		}
		version
	}

	pub fn run(&self, version: String, code: String, code_path: std::path::PathBuf, flags: Flags, ansi_enabled: bool) {
		match version.as_str() {
			"0.1.0" => {
				if flags.debug {
					println!("{}Running version 0.1.0", crate::Utils::ansi_escape_text("94", "DEBUG", v0_1_0::INFO_PREFIX_LENGTH, ansi_enabled));
				};
				v0_1_0::Runner::new(code, code_path, flags, ansi_enabled).run()
			}
			"0.2.0" => {
				if flags.debug {
					println!("{}Running version 0.2.0", crate::Utils::ansi_escape_text("94", "DEBUG", v0_2_0::INFO_PREFIX_LENGTH, ansi_enabled));
				};
				v0_2_0::Runner::new(code, code_path, flags, ansi_enabled).run()
			}
			_ => panic!("Couldn't launch version {}", &version),
		}
	}
}
