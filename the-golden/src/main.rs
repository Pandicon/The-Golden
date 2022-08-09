use dotenv::dotenv;
use std::env;

#[path = "./flags.rs"]
mod flags;
pub use flags::Flags;
#[path = "./interpreter/interpreter.rs"]
mod interpreter;
use interpreter::Interpreter;
#[path = "./utils.rs"]
mod utils;
pub use utils::Utils;

pub const INFO_PREFIX_LENGTH: usize = 12;

fn main() {
	dotenv().ok();
	if env::var("RUST_LOG").is_err() {
		env::set_var("RUST_LOG", "INFO");
	}
	tracing_subscriber::fmt::init();
	let ansi_enabled = enable_ansi_support::enable_ansi_support().is_ok();
	let args: Vec<String> = std::env::args().collect();

	let mut flags_handler = Flags::new();
	flags_handler.parse(&args);

	let mut _action = String::new();
	let mut version = String::from("latest");
	let mut code = String::new();
	let mut code_path = std::path::PathBuf::new();

	let cloned_flags = flags_handler.clone();
	if let Some(a) = cloned_flags.action {
		_action = a;
	}
	if let Some(path) = cloned_flags.code_path {
		code = match std::fs::read_to_string(&path) {
			Ok(c) => c,
			Err(e) => {
				println!(
					"{}Couldn't open a maumivu.au file from the provided path: {}",
					Utils::ansi_escape_text("91", "ERROR", INFO_PREFIX_LENGTH, ansi_enabled),
					e
				);
				return;
			}
		};
		code_path = path;
	} else if let Some(code_to_run) = cloned_flags.raw_code_to_run {
		code = code_to_run;
		code_path.set_file_name("<console_input_main>");
	}
	if code.is_empty() {
		println!(
			"{}No code provided - either provide a path to the maumivu.au file, or use the '- <code>' flag to run code from the command line directly",
			Utils::ansi_escape_text("91", "ERROR", INFO_PREFIX_LENGTH, ansi_enabled)
		);
		return;
	}
	if let Some(v) = cloned_flags.version {
		version = v;
	}
	if let Ok(val) = env::var("LOGS") {
		if val.to_lowercase() == "off" && cfg!(target_os = "windows") {
			winconsole::window::hide();
		}
	}
	if cloned_flags.no_console && cfg!(target_os = "windows") {
		winconsole::window::hide();
	}
	Interpreter::new(version, code, code_path, flags_handler, ansi_enabled).run();
}
