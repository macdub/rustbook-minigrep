use std::env;
use std::error::Error;
use std::fs;
use std::fmt;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    println!("{}", &cfg);

    // read the contents of the file
    let contents = fs::read_to_string(cfg.file)?; // condense to a huh-operator; get the error if present

    // switch function call based on b_ignore_case
    let results = if cfg.b_ignore_case {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search_case_sensitive(&cfg.query, &contents)
    };

    // iterate over 'results'
    for line in results{
        println!("{line}"); // print the line(s) that were found
    }

    Ok(()) // no error, all good here.
}

// this 'pub' stuff could become very tedious, but
// I suppose that it helps with the safety of everything.
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file: String,
    pub b_ignore_case: bool     // prepending 'b_' to make it easier to quickly tell this is a bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // I know you can just use the position, but this is more explicit
        Ok(Config{ query: args[1].clone(), file: args[2].clone(), b_ignore_case: ignore_case })
    }
}

// this is outside the scope for this project, but really wanted an easy way to
// print the object now for debug purposes. I am pretty sure that there are things
// that are wrong or at least do not follow the rust patterns. should update once
// I get through the trait chapter.
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query: {}, file: {}, ignore case: {}", &self.query, &self.file, &self.b_ignore_case)
    }
}

/*
    setting the lifetime relative to the contents as the return will reference
    stuff in it and those references need to last as long as the contents does
    and no longer.
*/
pub fn search<'a>(query: &str, contents: &'a str,) -> Vec<&'a str> {
    // store the results in a vector
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<&'a str> {
    // store the results in a vector
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query){
            results.push(line);
        }
    }

    results
}

// the original search is case-sensitive so let's just use that
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search(query, contents)
}

/** tests **/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
       let query = "duct";
       let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        )
    }
}