use crate::Flags;

#[path = "./versions/handler.rs"] mod versions_handler;

pub struct Interpreter {
	flags: Flags,

	version: String,
	versions_handler: versions_handler::Handler,
	code: String,
	code_path: std::path::PathBuf
}

impl Interpreter {
	pub fn new(mut version: String, code: String, code_path: std::path::PathBuf, flags: Flags) -> Self {
		let versions_handler = versions_handler::Handler::new();
		version = versions_handler.parse_version(version);
		Self { flags, code, version, versions_handler, code_path }
	}

	pub fn run(&self) {
		self.versions_handler.run(self.version.clone(), self.code.clone(), self.code_path.clone(), self.flags.clone());
	}
}