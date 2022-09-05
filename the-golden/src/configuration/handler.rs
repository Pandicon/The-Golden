use serde::{Deserialize, Serialize};

use crate::Utils;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfigHandler {
	pub commands: CommandsConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CommandsConfig {
	pub version_commands_configs: std::collections::HashMap<String, VersionCommandsConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VersionCommandsConfig {
	pub chain_regex: String,
	pub local_regex: String,
	pub commands: Vec<CommandConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CommandConfig {
	pub command: String,
	pub regex: String,
	pub chainable: bool,
	pub chain_optimisable: bool,
	pub can_be_local: bool,
}

impl ConfigHandler {
	pub fn new(ansi_enabled: bool, config: &str) -> Option<Self> {
		match serde_json::from_str(config) {
			Ok(val) => Some(val),
			Err(e) => {
				println!(
					"{}Failed to load the commands config: {}",
					Utils::ansi_escape_text("91", "ERROR", crate::INFO_PREFIX_LENGTH, ansi_enabled),
					e
				);
				None
			}
		}
	}
}
