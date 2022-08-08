use crate::Flags;

#[path = "./v0-1-0/main.rs"]
mod v0_1_0;
#[path = "./v0-2-0/main.rs"]
mod v0_2_0;

pub struct Handler {
	versions: Versions,
}

impl Handler {
	pub fn new() -> Self {
		let versions_0 = Version::new(
			String::from("0"),
			vec![
				Version::new(String::from("1"), vec![Version::new(String::from("0"), vec![])]),
				Version::new(String::from("2"), vec![Version::new(String::from("0"), vec![])]),
			],
		);
		let versions = Versions::new(vec![versions_0]);
		Self { versions }
	}

	pub fn parse_version(&self, mut version: String) -> String {
		version = version.to_lowercase();
		if version == *"latest" {
			version = "x.x.x".to_string();
		}
		let mut parts = version.split('.').map(|x| x.to_string()).collect::<Vec<String>>();
		parts.truncate(3);
		for _ in 0..3 - parts.len() {
			parts.push(String::from("x"));
		}
		let mut version_parsed: Vec<String> = vec![];
		let mut current_subversion = if parts[0].parse::<i32>().is_err() {
			self.versions.versions.last().unwrap()
		} else {
			let mut to_return: Option<&Version> = None;
			for subversion in &self.versions.versions {
				if subversion.value == parts[0] {
					to_return = Some(subversion);
					break;
				}
			}
			if to_return.is_some() {
				to_return.unwrap()
			} else {
				self.versions.versions.last().unwrap()
			}
		};
		version_parsed.push(current_subversion.value.clone());
		for ver in &parts[1..3] {
			if ver.parse::<i32>().is_err() {
				current_subversion = current_subversion.sub.last().unwrap();
			} else {
				let mut to_return: Option<&Version> = None;
				for subversion in current_subversion.sub.as_ref() {
					if subversion.value == *ver {
						to_return = Some(subversion);
						break;
					}
				}
				if to_return.is_some() {
					current_subversion = to_return.unwrap()
				} else {
					current_subversion = current_subversion.sub.last().unwrap()
				}
			}
			version_parsed.push(current_subversion.value.clone());
		}
		version_parsed.join(".")
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

struct Versions {
	versions: Vec<Version>,
}

impl Versions {
	fn new(versions: Vec<Version>) -> Self {
		Self { versions }
	}
}

struct Version {
	sub: Box<Vec<Version>>,
	value: String,
}

impl Version {
	fn new(value: String, sub: Vec<Version>) -> Self {
		Self { sub: Box::new(sub), value }
	}
}
