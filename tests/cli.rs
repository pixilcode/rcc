use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::path;

/// The directory for all test assets,
/// including code files and projects
/// 
/// NOTE: this is a macro in order to
/// produce a literal string for the
/// `concat!` macro
macro_rules! test_assets_dir {
	() => { concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets") }
}

/// The project directory containing
/// project compilation assets such as
/// the executable
/// 
/// NOTE: this is a macro in order to
/// produce a literal string for the
/// `concat!` macro
macro_rules! project_assets_dir {
	() => { "/.rcc-target" }
}

#[test]
fn compile_single_file() -> Result<(), Box<dyn std::error::Error>> {
	// Set up the command
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?; // Run the current bin
	cmd.current_dir(concat!(test_assets_dir!(), "/single-file")); // Set the directory
	cmd.arg("run").arg("hello-world.cpp"); // Run the `hello-world.cpp` file

	// Run the command
	cmd.assert()
		.success() // Assert that it was a success
		//.stdout(predicate::eq("Success").trim()) // 
	;

	let executable_path = concat!(
		test_assets_dir!(), "/single-file", project_assets_dir!(), "/hello-world"
	);

	assert!(path::Path::new(executable_path).exists(), "executable doesn't exist");

	Ok(())
}

