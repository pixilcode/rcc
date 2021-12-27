use std::path;
use std::process;
use std::fmt;
use std::io::{self, Write};
use std::ffi::OsString;
use std::fs;

/// The subdirectory in which all compilation
/// artifacts will be placed
const TARGET_DIR: &str = ".rcc-target";

/// Compile the give file and produce an executable
/// 
/// The `PathBuf` that is returned is the path to
/// the executable
pub fn compile<P: AsRef<path::Path>>(path: P) -> Result<path::PathBuf, CompileError> {
	// Get the absolute path of the file
    let path = match path.as_ref().canonicalize() {
        Ok(path) => path,
        Err(_) => return Err(CompileError::InvalidPath(path.as_ref().into())),
    };

    // Check to make sure that it is a file
    if !path.is_file() {
        return Err(CompileError::NotFile(
			path.file_name().map(|s| s.to_owned())
		));
    }

    // Check to make sure that the extension is '.cpp' or '.c'
    match path.extension() {
        Some(ext)
            if ext == "cpp" || ext == "c" => (),
        ext => return Err(CompileError::InvalidExtension(
			ext.map(|s| s.to_owned())
		)),
    }

	// The directory of the file
	let dir_path = path.parent()
		.expect("error: please rerun the command"); // Already made sure it was a file and thus has a parent

	// The executable name is the name of the file
	// minus the extension
	let exe_name = path.file_stem()
		.expect("error: please rerun the command"); // Already checked to see if it was a file

	// The path to the executable (in a hidden directory)
	let exe_dir = dir_path.join(TARGET_DIR);
	let exe_path = dir_path.join(TARGET_DIR).join(exe_name);

	// Build the directory if it doesn't exist
	if !exe_dir.exists() {
		fs::create_dir(exe_dir)
			.expect("error: please rerun the command"); // This will work because the original path is a valid path
	}
    
	// Build the command
	let mut compile_cmd = process::Command::new("g++");
	
	compile_cmd
		.arg(&path)
		.arg("-o")
		.arg(&exe_path);

	// Run the command
	let output = match compile_cmd.output() {
		Ok(output) => output,
		Err(_) => return Err(CompileError::GppError),
	};

	// Write the output standard output
	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();

	if output.status.success() {
		Ok(exe_path)
	} else {
		Err(CompileError::GppCompileError)
	}
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
/// Errors that can be encountered when running a file
pub enum CompileError {
    /// The path is invalid
    InvalidPath(path::PathBuf),
    
    /// The path doesn't point to a file
    NotFile(Option<OsString>),

    /// The extension is invalid (not `.c` or `.cpp`)
    InvalidExtension(Option<OsString>),

	/// There was an error running `g++`
	GppError,

	/// There was an error in compiling the file using `g++`
	GppCompileError,
}

impl std::error::Error for CompileError {}

impl fmt::Display for CompileError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CompileError::InvalidPath(path) => write!(f, "error: '{}' is not a valid path", path.to_string_lossy()),
			CompileError::NotFile(Some(name)) => write!(f, "error: '{}' is not a file", name.to_string_lossy()),
			CompileError::NotFile(None) => write!(f, "error: not a file"),
			CompileError::InvalidExtension(_) => write!(f, "error: file must have a `.cpp` or `.c` extension"),
			CompileError::GppError => write!(f, "error: couldn't run 'g++' (make sure it is installed)"),
			CompileError::GppCompileError => write!(f, "error: code didn't compile (see compiler output above)"),
		}
	}
}