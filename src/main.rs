use structopt::StructOpt;
use std::path;
use std::process;
use std::io::Write;

mod compile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();

    match opts {
        Opt::Run { path } => run_file(path),
    }
}

fn run_file<P: AsRef<path::Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let exec_path = compile::compile(path)?;
    run_exec(exec_path)?;
    
    Ok(())
}

fn run_exec<P: AsRef<path::Path>>(exec_path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut exec_cmd = process::Command::new(exec_path.as_ref().as_os_str());

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
        path: path::PathBuf
    }
}
