use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, process};

/// Denotes forms in the target.
#[derive(Debug, Eq, PartialEq)]
struct Form<'a> {
    name: &'a str,
}

impl<'a> Form<'a> {
    fn new(name: &str) -> Form {
        Form { name }
    }
}

/// Show usage.
fn usage() {
    println!("USAGE");
    println!("    fill INPUT");
}

fn show<P: AsRef<Path>>(path: P) {
    let input = File::open(path).unwrap();
    let buf = BufReader::new(&input);

    for l in buf.lines() {
        println!("{}", l.unwrap());
    }
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        show(file_path);
    } else {
        usage();
        process::exit(1);
    }

    process::exit(0);
}

/// Get forms from the input.
/// Returns empty vector if the input does not contain forms.
fn get_forms(input: &str) -> Vec<Form> {
    let re = Regex::new(r"\{(.*?)\}").unwrap();
    let mut result = vec![];

    for name in re.captures_iter(input) {
        result.push(Form::new(name.get(1).unwrap().as_str()));
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_forms() {
        // Single form
        assert_eq!(
            get_forms("Hello, my name is {name}."),
            vec![Form::new("name")]
        );

        // Multiple forms
        assert_eq!(
            get_forms("Hello, my name is {name}. I'm {age} and live in {place}."),
            vec![Form::new("name"), Form::new("age"), Form::new("place")]
        );

        // None
        assert_eq!(
            get_forms("Hello"),
            vec![]
        );

        // Multiple lines
        assert_eq!(
            get_forms(
                "Hello, my name is {name}.
                      I'm {age} and live in {place}."
            ),
            vec![Form::new("name"), Form::new("age"), Form::new("place")]
        );
    }
}
