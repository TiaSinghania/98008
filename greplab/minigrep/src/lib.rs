pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query_case: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    let query = query_case.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line)
        }
    }
    res
}

pub fn search_count(query: &str, contents: &str) -> usize {
    let mut res = 0;
    for line in contents.lines() {
        if line.contains(query) {
            res += 1
        }
    }
    res
}

pub fn search_case_insensitive_count(query_case: &str, contents: &str) -> usize {
    let mut res = 0;
    let query = query_case.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res += 1
        }
    }
    res
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
    fn case_insensitive() {
        let query = "RuSt";
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

    #[test]
    fn count_flag_case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(2, search_case_insensitive_count(query, contents));
    }

    #[test]
    fn count_flag_case_sensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(0, search_count(query, contents));
    }

    // The below test cases were generated using ChatGPT. They were reviewed manually to ensure correctness.

    #[test]
    fn no_matches() {
        let query = "Python";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(Vec::<&str>::new(), search(query, contents));
        assert_eq!(0, search_count(query, contents));
    }

    #[test]
    fn multiple_matches_same_line() {
        let query = "Rust";
        let contents = "Rust is great. Rust is fast. Rust rules!";
        assert_eq!(
            vec!["Rust is great. Rust is fast. Rust rules!"],
            search(query, contents)
        );
        assert_eq!(1, search_count(query, contents)); // counts per line, not per occurrence
    }

    #[test]
    fn matches_on_empty_input() {
        let query = "Rust";
        let contents = "";
        assert_eq!(Vec::<&str>::new(), search(query, contents));
        assert_eq!(0, search_count(query, contents));
    }

    #[test]
    fn empty_query_should_match_everything() {
        let query = "";
        let contents = "\
Line one
Line two
Line three";

        // Every line contains the empty string
        assert_eq!(
            vec!["Line one", "Line two", "Line three"],
            search(query, contents)
        );
        assert_eq!(3, search_count(query, contents));
    }

    #[test]
    fn case_insensitive_mixed_case_lines() {
        let query = "rust";
        let contents = "\
RUST is cool
Rust is fun
rust rocks
Just trust it";
        assert_eq!(
            vec!["RUST is cool", "Rust is fun", "rust rocks", "Just trust it"],
            search_case_insensitive(query, contents)
        );
        assert_eq!(4, search_case_insensitive_count(query, contents));
    }

    #[test]
    fn whitespace_and_punctuation_handling() {
        let query = "rust";
        let contents = "   rust!   \nno match here\nrust?";
        assert_eq!(
            vec!["   rust!   ", "rust?"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_sensitive_partial_matches() {
        let query = "Ru";
        let contents = "\
Rust
rust
RUst";
        assert_eq!(vec!["Rust"], search(query, contents));
        assert_eq!(1, search_count(query, contents));
    }

    #[test]
    fn case_insensitive_partial_matches() {
        let query = "ru";
        let contents = "\
Rust
rust
RUst";
        assert_eq!(
            vec!["Rust", "rust", "RUst"],
            search_case_insensitive(query, contents)
        );
        assert_eq!(3, search_case_insensitive_count(query, contents));
    }
}
