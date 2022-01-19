use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::path;
use std::fs;

/// The directory for all test assets,
/// including code files and projects
macro_rules! test_dir {
	($dir:literal) => { concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets", $dir) }
}

/// The project directory containing
/// project compilation assets such as
/// the executable
macro_rules! project_assets_dir {

	($project_dir:literal) => {
		concat!(test_dir!($project_dir), "/.rcc-target")
	};

	($project_dir:literal, $executable:literal) => {
		concat!(test_dir!($project_dir), "/.rcc-target", $executable)
	}
}

#[test]
fn compile_single_file() -> Result<(), Box<dyn std::error::Error>> {
	// Set up the command
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?; // Run the current bin
	cmd.current_dir(test_dir!("/single-file")); // Set the directory
	cmd.arg("run").arg("hello-world.cpp"); // Run the `hello-world.cpp` file

	// Run the command
	cmd.assert()
		.success() // Assert that it was a success
		.stdout(predicate::eq("Success").trim()) // Ensure the correct output
	;

	let executable_dir = project_assets_dir!("/single-file");
	let executable_path = project_assets_dir!("/single-file", "/hello-world");

	assert!(path::Path::new(executable_path).exists(), "executable doesn't exist");

	// Cleanup
	fs::remove_dir_all(executable_dir).unwrap();

	Ok(())
}

#[test]
fn compile_single_file_with_args() -> Result<(), Box<dyn std::error::Error>> {
	// Set up the command
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?; // Run the current bin
	cmd.current_dir(test_dir!("/single-file-with-args")); // Set the directory
	cmd
		.arg("run").arg("read-arg.cpp") // Run the `read-arg.cpp` file
		.arg("--arg").arg("success") // Pass the executable the argument "success"
		.arg("--arg").arg("success-2");
	
	// Run the command
	cmd.assert()
		.success() // Assert that it was a success
		.stdout(predicate::eq("success success-2").trim()); // Ensure the correct output
	
	let executable_dir = project_assets_dir!("/single-file-with-args");
	let executable_path = project_assets_dir!("/single-file-with-args", "/read-arg");

	assert!(path::Path::new(executable_path).exists(), "executable doesn't exist");

	fs::remove_dir_all(executable_dir).unwrap();

	Ok(())
}
