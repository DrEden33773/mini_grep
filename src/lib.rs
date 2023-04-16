use std::{ffi::OsString, fs};

type SuperError = Box<dyn std::error::Error>;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [OsString]) -> Result<Self, SuperError> {
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
    pub fn run(&self) -> Result<(), SuperError> {
        let contents = fs::read_to_string(self.file_path)?;
        println!("With text:\n{}", contents);
        Ok(())
    }
}
