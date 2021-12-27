use structopt::StructOpt;
use std::path;

mod compile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();

    match opts {
        Opt::Run { path } => run_file(path),
    }
}

fn run_file<P: AsRef<path::Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    compile::compile(path)?;
    //todo!("run the file");
    
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
