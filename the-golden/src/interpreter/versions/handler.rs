use crate::{ConfigHandler, Flags};

#[path = "./v0-1-0/main.rs"]
mod v0_1_0;
#[path = "./v0-2-0/main.rs"]
mod v0_2_0;
#[path = "./v0-3-0/main.rs"]
mod v0_3_0;
#[path = "./v0-4-0/main.rs"]
mod v0_4_0;
#[path = "./v0-5-0/main.rs"]
mod v0_5_0;

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
				Version::new(String::from("3"), vec![Version::new(String::from("0"), vec![])]),
				Version::new(String::from("4"), vec![Version::new(String::from("0"), vec![])]),
			],
		);
		let versions = Versions::new(vec![versions_0]);
		Self { versions }
	}

	pub fn parse_version(&self, mut version: String, ansi_enabled: bool) -> String {
		let version_original = version.clone();
		if version.to_lowercase() == *"latest" {
			version = "x.x.x".to_string();
		}
		let mut parts = version.split('.').map(|x| x.to_string()).collect::<Vec<String>>();
		parts.truncate(3);
		for _ in 0..3 - parts.len() {
			parts.push(String::from("x"));
		}
		let s = parts[2].split('-').map(|x| x.to_string()).collect::<Vec<String>>();
		let (parts, prerelease, _build_metadata) = if s.len() > 1 {
			let s_joined = s[1..].join("-");
			let p = s_joined.clone();
			let s2 = p.split('+').map(|x| x.to_string()).collect::<Vec<String>>();
			if s2.len() > 1 {
				(vec![parts[0].clone(), parts[1].clone(), s[0].clone()], Some(s2[0].clone()), Some(s2[1..].join("+")))
			} else {
				(vec![parts[0].clone(), parts[1].clone(), s[0].clone()], Some(s_joined), None)
			}
		} else {
			(parts, None, None)
		};
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
			if let Some(to_return) = to_return {
				to_return
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
				for subversion in &current_subversion.sub {
					if subversion.value == *ver {
						to_return = Some(subversion);
						break;
					}
				}
				current_subversion = if let Some(to_return) = to_return { to_return } else { current_subversion.sub.last().unwrap() }
			}
			version_parsed.push(current_subversion.value.clone());
		}
		let prerelease = match (prerelease, !current_subversion.sub.is_empty()) {
			(Some(ver), true) => {
				let mut to_return: Option<&Version> = None;
				for subversion in &current_subversion.sub {
					if subversion.value == ver {
						to_return = Some(subversion);
						break;
					}
				}
				current_subversion = if let Some(to_return) = to_return { to_return } else { current_subversion.sub.last().unwrap() };
				format!("-{}", current_subversion.value.clone())
			}
			_ => String::new(),
		};
		let version_final = format!("{}{}", version_parsed.join("."), prerelease);
		if version_original != version_final && version_original.to_lowercase() != "latest" {
			println!(
				"{}Could not find version {}, instead found {}",
				crate::Utils::ansi_escape_text("93", "WARNING", v0_1_0::INFO_PREFIX_LENGTH, ansi_enabled),
				version_original,
				version_final
			);
		}
		version_final
	}

	pub fn run(&self, version: String, code: String, code_path: std::path::PathBuf, flags: Flags, ansi_enabled: bool, config_handler: ConfigHandler) {
		match version.as_str() {
			"0.1.0" => {
				if flags.debug {
					println!("{}Running version {}", crate::Utils::ansi_escape_text("94", "DEBUG", v0_1_0::INFO_PREFIX_LENGTH, ansi_enabled), version);
				};
				v0_1_0::Runner::new(code, code_path, flags, ansi_enabled).run()
			}
			"0.2.0" => {
				if flags.debug {
					println!("{}Running version {}", crate::Utils::ansi_escape_text("94", "DEBUG", v0_2_0::INFO_PREFIX_LENGTH, ansi_enabled), version);
				};
				v0_2_0::Runner::new(code, code_path, flags, ansi_enabled).run()
			}
			"0.3.0" => {
				if flags.debug {
					println!("{}Running version {}", crate::Utils::ansi_escape_text("94", "DEBUG", v0_3_0::INFO_PREFIX_LENGTH, ansi_enabled), version);
				};
				v0_3_0::Runner::new(code, code_path, flags, ansi_enabled).run()
			}
			"0.4.0" => {
				if flags.debug {
					println!("{}Running version {}", crate::Utils::ansi_escape_text("94", "DEBUG", v0_4_0::INFO_PREFIX_LENGTH, ansi_enabled), version);
				};
				v0_4_0::Runner::new(code, code_path, flags, ansi_enabled).run()
			}
			"0.5.0" => {
				if flags.debug {
					println!("{}Running version {}", crate::Utils::ansi_escape_text("94", "DEBUG", v0_5_0::INFO_PREFIX_LENGTH, ansi_enabled), version);
				};
				let version_commands = match config_handler.commands.version_commands_configs.get(&version) {
					Some(commands) => commands.clone(),
					None => {
						println!(
							"{}Couldn't get commands for version {}",
							crate::Utils::ansi_escape_text("91", "ERROR", v0_5_0::INFO_PREFIX_LENGTH, ansi_enabled),
							version
						);
						return;
					}
				};
				v0_5_0::Runner::new(code, code_path, flags, ansi_enabled, version_commands).run()
			}
			_ => {
				println!(
					"{}Couldn't run version {}",
					crate::Utils::ansi_escape_text("91", "ERROR", v0_1_0::INFO_PREFIX_LENGTH, ansi_enabled),
					version
				);
			}
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

#[derive(Debug)]
struct Version {
	sub: Vec<Version>,
	value: String,
}

impl Version {
	fn new(value: String, sub: Vec<Version>) -> Self {
		Self { sub, value }
	}
}
