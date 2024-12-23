use std::fs::read_to_string;

pub struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Config {
        let query = args.get(1).expect("请提供查询字符串");
        let filename = args.get(2).map_or("", |x| x.as_str());
        Config {
            query,
            filename,
        }
    }

    pub fn search<'a>(&self, content: &'a str) -> Result<Vec<&'a str>, Box<dyn std::error::Error>> {
        let find_lines: Vec<&str> = content
            .lines()
            .filter(|line| line.contains(self.query))
            .collect();
        Ok(find_lines)
    }
}

pub fn search<'a>(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string(cfg.filename)?;
    let res = find_lines(&content, cfg.query);
    for (line, start) in res {
        highligh_println(line, start, cfg.query.len());
    }
    Ok(())
}

pub fn search_from_input(input: &str, cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let res: Vec<(&str, usize)> = find_lines(input, cfg.query);
    for (line, start) in res {
        highligh_println(line, start, cfg.query.len());
    }
    Ok(())
}

fn highligh_println(input: &str, start: usize, len: usize) {
    let before = &input[..start];
    let matched = &input[start..start + len];
    let after = &input[start + len..];
    println!("{}{}{}", before, format!("\x1b[31m{}\x1b[0m", matched), after);
}

fn find_lines<'a>(content: &'a str, query: &str) -> Vec<(&'a str, usize)> {
    content
        .lines()
        .filter_map(|line| {
            if let Some(pos) = line.find(query) {
                Some((line, pos))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lines() {
        let content = "I'm a line\nI'm another line\nI'm the last line";
        let query = "line";
        let res = find_lines(content, query);
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_find_lines_empty() {
        let content = "I'm a line\nI'm another line\nI'm the last line";
        let query = "nothere";
        let res = find_lines(content, query);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_search_ok() {
        let cfg = Config {
            query: "im",
            filename: "test/a.txt",
        };
        assert_eq!(search(cfg).is_ok(), true);
    }

    #[test]
    fn test_search_empty() {
        let cfg = Config {
            query: "nothere",
            filename: "test/a.txt",
        };
        assert_eq!(search(cfg).is_ok(), true);
    }

    #[test]
    fn test_search_fail() {
        let cfg = Config {
            query: "im",
            filename: "test/b.txt",
        };
        assert_eq!(search(cfg).is_err(), true);
    }
}
