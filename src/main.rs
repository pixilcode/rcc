use structopt::StructOpt;
use std::path;
use std::process;
use std::io::Write;
use std::ffi::OsString;

mod compile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();

    match opts {
        Opt::Run { path, args } => run_file(path, args),
    }
}

fn run_file<P: AsRef<path::Path>>(path: P, args: Option<Vec<OsString>>) -> Result<(), Box<dyn std::error::Error>> {
    let exec_path = compile::compile(path)?;
    run_exec(exec_path, args)?;
    
    Ok(())
}

fn run_exec<P: AsRef<path::Path>>(exec_path: P, args: Option<Vec<OsString>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut exec_cmd = process::Command::new(exec_path.as_ref().as_os_str());

    if let Some(args) = args {
        exec_cmd.args(args);
    }

    let output = exec_cmd.output()?;

    // Write the output standard output
	std::io::stdout().write_all(&output.stdout).unwrap();
	std::io::stderr().write_all(&output.stderr).unwrap();

    Ok(())
}

#[derive(StructOpt)]
enum Opt {
    /// Run a file
    Run {
        /// The path to the file
        path: path::PathBuf,

        /// Args to pass to the executable
        #[structopt(short = "a", long = "arg")]
        args: Option<Vec<OsString>>
    }
}
