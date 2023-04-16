use mini_grep::Config;
use std::{env, ffi::OsString, process};

type SuperError = Box<dyn std::error::Error>;

pub fn main() -> Result<(), SuperError> {
    let args: Vec<OsString> = env::args_os().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(-1);
    });
    println!("Searching for: {}", config.query);
    println!("In file `{}`", config.file_path);
    if let Err(err) = config.run() {
        println!("Application error: {err}");
        process::exit(0);
    }
    Ok(())
}
