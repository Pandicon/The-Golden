use crate::Flags;

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
	pub fn new(mut version: String, code: String, code_path: std::path::PathBuf, flags: Flags, ansi_enabled: bool) -> Self {
		let versions_handler = versions_handler::Handler::new();
		version = versions_handler.parse_version(version, ansi_enabled);
		Self {
			flags,
			ansi_enabled,
			code,
			version,
			versions_handler,
			code_path,
		}
	}

	pub fn run(&self) {
		self.versions_handler
			.run(self.version.clone(), self.code.clone(), self.code_path.clone(), self.flags.clone(), self.ansi_enabled);
	}
}
