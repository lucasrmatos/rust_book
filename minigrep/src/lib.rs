
use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    // query == buscar
    pub query: String,
    // file_path == nome do arquivo (deve estar na raiz do projeto)
    pub file_path: String,
    pub ignore_case: bool,

}
impl Config{
#[allow(dead_code)]
    fn new (args: &[String]) -> Config{
        if args.len() < 3{
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config{ query, file_path,ignore_case : false }
    }

    //                 esse impl abaixo é trait bound. Means that args can be any type that implements the Iterator trait and returns String items.
    pub fn build (mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {
        
        //Remember that the first value in the return value of env::args is the name of the program. We want to ignore that and get to the next value, so first we call next and do nothing with the return value.
        args.next();

        //Second, we call next to get the value we want to put in the query field of Config. If next returns a Some, we use a match to extract the value. If it returns None, it means not enough arguments were given and we return early with an Err value. We do the same thing for the file_path value.
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })

    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

//Removing the mutable state might enable a future enhancement to make searching happen in parallel, because we wouldn’t have to manage concurrent access to the results vector. 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query)) //this code uses the filter adapter to keep only the lines that line.contains(query) returns true.
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}

