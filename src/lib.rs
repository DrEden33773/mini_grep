use std::{env, error::Error, ffi::OsString, fs};

pub struct Config {
    query: String,
    file_path: String,
    if_ignore_case: bool,
}

impl Config {
    pub fn build_from_args(
        query: &OsString,
        file_path: &OsString,
        if_ignore_case: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let (query, file_path) = (
            query.to_str().ok_or("Invalid `query`!")?.to_string(),
            file_path
                .to_str()
                .ok_or("Invalid `file_path`!")?
                .to_string(),
        );
        Ok(Config {
            query,
            file_path,
            if_ignore_case,
        })
    }
    pub fn build_from_iter<T>(mut args: T) -> Result<Self, Box<dyn Error>>
    where
        T: Iterator<Item = OsString>,
    {
        args.next();
        let (query, file_path) = (
            args.next()
                .ok_or("Can't find `query` arg!")?
                .to_str()
                .ok_or("Invalid `query`!")?
                .to_string(),
            args.next()
                .ok_or("Can't find `file_path` arg!")?
                .to_str()
                .ok_or("Invalid `file_path`!")?
                .to_string(),
        );
        let if_ignore_case = env::var("IGNORE_CASE").is_ok()
            || if let Some(_) = args.next() {
                true
            } else {
                false
            };
        Ok(Config {
            query,
            file_path,
            if_ignore_case,
        })
    }
    pub fn build(args: &[OsString]) -> Result<Self, Box<dyn Error>> {
        let (query, file_path) = (
            (&args
                .get(1)
                .ok_or("Can't find `query` arg!")?
                .to_str()
                .ok_or("Invalid `query`!")?
                .to_string())
                .to_owned(),
            (&args
                .get(2)
                .ok_or("Can't find `file_path` arg!")?
                .to_str()
                .ok_or("Invalid `file_path`!")?
                .to_string())
                .to_owned(),
        );
        let if_ignore_case = env::var("IGNORE_CASE").is_ok()
            || if let Some(_) = &args.get(3) {
                true
            } else {
                false
            };
        Ok(Config {
            query,
            file_path,
            if_ignore_case,
        })
    }
    pub fn show_all_contents(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.file_path)?;
        println!("\nSearching for `{}`", &self.query);
        println!("In file `{}`", &self.file_path);
        println!("With text:\n");
        println!("{}", contents);
        Ok(())
    }
    pub fn search(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let contents = fs::read_to_string(&self.file_path)?;
        let results = contents
            .lines()
            .filter(|line| line.contains(&self.query))
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        Ok(results)
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        fn search<'a>(query: &str, contents: &'a str, if_ignore_case: bool) -> Vec<&'a str> {
            contents
                .lines()
                .filter(|line| {
                    if if_ignore_case {
                        line.to_lowercase().contains(query)
                    } else {
                        line.contains(query)
                    }
                })
                .collect::<Vec<_>>()
        }
        println!("\nSearching for `{}`", &self.query);
        println!("In file `{}`", &self.file_path);
        println!("With text:\n");
        let contents = fs::read_to_string(&self.file_path)?;
        search(&self.query, &contents, self.if_ignore_case)
            .into_iter()
            .for_each(|line| {
                println!("{line}");
            });
        Ok(())
    }
}
