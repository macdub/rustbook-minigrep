use std::error::Error;
use std::fs;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    // read the contents of the file
    let contents = fs::read_to_string(cfg.file)?; // condense to a huh-operator; get the error if present

    for line in search(&cfg.query, &contents){
        println!("{line}"); // save the live
    }

    Ok(()) // no error, all good here.
}

// this 'pub' stuff could become very tedious, but
// I suppose that it helps with the safety of everything.
pub struct Config {
    pub query: String,
    pub file: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config{ query: args[1].clone(), file: args[2].clone() })
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
}