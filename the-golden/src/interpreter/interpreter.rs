use crate::Flags;

#[path = "./preprocessor.rs"]
mod preprocessor;
#[path = "./versions/handler.rs"]
mod versions_handler;

pub struct Interpreter {
	flags: Flags,

	ansi_enabled: bool,
	version: String,
	versions_handler: versions_handler::Handler,
	code: String,
	code_path: std::path::PathBuf,
}

impl Interpreter {
	pub fn new(version: Option<String>, code: String, code_path: std::path::PathBuf, mut flags: Flags, ansi_enabled: bool) -> Self {
		let mut preprocessor = preprocessor::Preprocessor::new();
		preprocessor.run(&code);
		flags.no_console |= preprocessor.no_console;
		let final_version = if let Some(ver) = version {
			ver
		} else if let Some(ver) = preprocessor.version {
			ver
		} else {
			String::from("latest")
		};
		if flags.sebek.iter().filter(|val| val.is_some()).collect::<Vec<&Option<f64>>>().is_empty() {
			flags.sebek = preprocessor.sebek;
		};
		let versions_handler = versions_handler::Handler::new();
		let parsed_version = versions_handler.parse_version(final_version, ansi_enabled);

		Self {
			flags,
			ansi_enabled,
			code,
			version: parsed_version,
			versions_handler,
			code_path,
		}
	}

	pub fn run(&self) {
		self.versions_handler
			.run(self.version.clone(), self.code.clone(), self.code_path.clone(), self.flags.clone(), self.ansi_enabled);
	}
}
