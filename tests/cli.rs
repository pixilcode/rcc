use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn compile_single_file() -> Result<(), Box<dyn std::error::Error>> {
	// Set up the command
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?; // Run the current bin
	cmd.current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets/single-file")); // Set the directory
	cmd.arg("run").arg("hello-world.cpp"); // Run the `hello-world.cpp` file

	// Run the command
	cmd.assert()
		.success() // Assert that it was a success
		.stdout(predicate::eq("Success").trim()); // 

	Ok(())
}