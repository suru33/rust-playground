use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    case_sensitive: bool,
}

impl Config {
    const ERR_MSG: &'static str = "Invalid arguments";

    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() == 3 {
            let query = args[1].clone();
            let file_name = args[2].clone();

            Ok(Config {
                query,
                file_name,
                case_sensitive: true,
            })
        } else if args.len() == 4 {
            let flag = args[1].clone();
            let query = args[2].clone();
            let file_name = args[3].clone();

            if flag.eq("-i") {
                Ok(Config {
                    query,
                    file_name,
                    case_sensitive: false,
                })
            } else {
                Err(Config::ERR_MSG)
            }
        } else {
            Err(Config::ERR_MSG)
        }
    }
}

#[derive(Debug, PartialEq)]
struct QueryResult {
    line: u32,
    text: String,
}

impl QueryResult {
    fn print(&self) {
        println!("{:4}: {}", self.line, self.text)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_name)?;
    let results = if config.case_sensitive {
        search_v2(&config.query, &content)
    } else {
        search_case_insensitive_v2(&config.query, &content)
    };

    results.iter().for_each(QueryResult::print);

    Ok(())
}

fn search(query: &str, contents: &str) -> Vec<QueryResult> {
    let mut results: Vec<QueryResult> = Vec::new();
    for (ix, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push(QueryResult {
                line: (ix + 1) as u32,
                text: line.to_string(),
            });
        }
    }
    results
}

fn search_v2(query: &str, contents: &str) -> Vec<QueryResult> {
    let results: Vec<QueryResult> = contents
        .lines()
        .enumerate()
        .filter(|(_ix, text)| text.contains(query))
        .map(|(ix, text)| QueryResult {
            line: (ix + 1) as u32,
            text: text.to_string(),
        })
        .collect();

    results
}

fn search_case_insensitive(query: &str, contents: &str) -> Vec<QueryResult> {
    let query = query.to_lowercase();
    let mut results: Vec<QueryResult> = Vec::new();
    for (ix, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push(QueryResult {
                line: (ix + 1) as u32,
                text: line.to_string(),
            });
        }
    }
    results
}

fn search_case_insensitive_v2(query: &str, contents: &str) -> Vec<QueryResult> {
    let query = query.to_lowercase();
    let results: Vec<QueryResult> = contents
        .lines()
        .enumerate()
        .filter(|(_ix, text)| text.to_lowercase().contains(&query))
        .map(|(ix, text)| QueryResult {
            line: (ix + 1) as u32,
            text: text.to_string(),
        })
        .collect();

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let expected = vec![QueryResult {
            line: 2,
            text: "safe, fast, productive.".to_string(),
        }];

        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let expected = vec![QueryResult {
            line: 1,
            text: "Rust:".to_string(),
        }];

        assert_eq!(expected, search_case_insensitive(query, contents));
    }
}
