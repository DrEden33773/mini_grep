use std::{env, error::Error, ffi::OsString, fs};

type SuperError = Box<dyn Error>;

fn main() -> Result<(), SuperError> {
    let args: Vec<OsString> = env::args_os().collect();
    let config = Config::new(&args)?;
    println!("Searching for: {}", config.query);
    println!("In file `{}`", config.file_path);
    let contents = fs::read_to_string(config.file_path).expect("Can't read file!");
    println!("With text:\n{}", contents);
    Ok(())
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [OsString]) -> Result<Self, SuperError> {
        let (query, file_path) = (
            &args
                .get(1)
                .ok_or("Can't find `query` arg!")?
                .to_str()
                .ok_or("Invalid query!")?,
            &args
                .get(2)
                .ok_or("Can't find `file_path` arg!")?
                .to_str()
                .ok_or("Invalid file_path!")?,
        );
        Ok(Config { query, file_path })
    }
}
