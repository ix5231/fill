use regex::Regex;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, process};

/// Input text.
struct Text<'a> {
    input: Cow<'a, str>,
}

impl<'a> Text<'a> {
    // Create `Text`.
    fn new(input: &str) -> Text {
        Text {
            input: Cow::from(input),
        }
    }

    /// Get forms from the input.
    /// Returns empty vector if the input does not contain forms.
    fn get_forms(&self) -> Vec<Form> {
        let re = Regex::new(r"\{(.*?)\}").unwrap();
        let mut result = vec![];

        for name in re.captures_iter(&self.input) {
            result.push(Form::new(
                name.get(1).unwrap().as_str().to_string(),
                name.get(0).unwrap().as_str().to_string(),
            ));
        }

        result
    }

    fn fill_form(&mut self, form: &Form, replace_text: &str) {
        self.input = Cow::from(self.input.replace(&form.form, replace_text));
    }
}

/// Forms in the target.
#[derive(Debug, Eq, PartialEq)]
struct Form {
    name: String,
    form: String,
}

impl Form {
    fn new(name: String, form: String) -> Form {
        Form { name, form }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_forms() {
        // Single form
        assert_eq!(
            Text::new("Hello, my name is {name}.").get_forms(),
            vec![Form::new("name".to_string(), "{name}".to_string())]
        );

        // Multiple forms
        assert_eq!(
            Text::new("Hello, my name is {name}. I'm {age} and live in {place}.").get_forms(),
            vec![
                Form::new("name".to_string(), "{name}".to_string()),
                Form::new("age".to_string(), "{age}".to_string()),
                Form::new("place".to_string(), "{place}".to_string())
            ]
        );

        // None
        assert_eq!(Text::new("Hello").get_forms(), vec![]);

        // Multiple lines
        assert_eq!(
            Text::new(
                "Hello, my name is {name}.
                      I'm {age} and live in {place}."
            )
            .get_forms(),
            vec![
                Form::new("name".to_string(), "{name}".to_string()),
                Form::new("age".to_string(), "{age}".to_string()),
                Form::new("place".to_string(), "{place}".to_string())
            ]
        );
    }

    #[test]
    fn test_fill_form() {
        let mut text = Text::new(
            "Hello, my name is {name}.
                      I'm {age} and live in {place}.",
        );
        let mut forms = text.get_forms();
        assert_eq!(
            forms,
            vec![
                Form::new("name".to_string(), "{name}".to_string()),
                Form::new("age".to_string(), "{age}".to_string()),
                Form::new("place".to_string(), "{place}".to_string())
            ]
        );

        let input = ["Ix5231", "19", "Japan"];
        for (form, i) in forms.iter_mut().zip(&input) {
            text.fill_form(form, i);
        }
        assert_eq!(
            text.input,
            "Hello, my name is Ix5231.
                      I'm 19 and live in Japan.",
        )
    }
}
