use std::{fs, error};

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn test_search() {
        let query = "Pick";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(search(&query.to_string(), &content.to_string()), vec!["Pick three."]);
    }
}

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    // new config
    pub fn new(list: &Vec<String>) -> Result<Config, &'static str> {
        if list.len() < 3 {
            return Err("param not enough");
        }
        Ok(
            Config {
                query: list.get(1).unwrap().clone(),
                file_name: list.get(2).unwrap().clone(),
            }
        )
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(&config.file_name)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &String, content: &'a String) -> Vec<&'a str> {
    content.lines()
        .filter(|x| x.contains(query))
        .collect()
}

