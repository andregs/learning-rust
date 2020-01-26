use std::error::Error;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    // in previous version of this code we didn't have ownership of "args", but now we do
    pub fn new<I>(mut args: I) -> Result<Config, &'static str> 
        where I: Iterator<Item = String>
    {
        args.next(); // discard 1st (the app bin path)
        
        // in this version we can return Config with owned values without cloning
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected: query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected: filename."),
        };
        
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
    
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        isearch(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// returns parts of "contents" that matches the "query", so the returned references
// must live as long as "contents"
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn isearch<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
