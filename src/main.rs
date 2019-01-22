use regex::Regex;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use clap::{App, Arg};

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

    /// Fill the form with input.
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
    /// Create `Form`.
    fn new(name: String, form: String) -> Form {
        Form { name, form }
    }
}

/// Fill the text interactively.
fn fill<P: AsRef<Path>>(src_path: P, dst_path: P) {
    let mut src = File::open(src_path).unwrap();
    let input = &mut String::new();
    src.read_to_string(input).unwrap();

    let mut text = Text::new(input);
    let forms = text.get_forms();

    let stdin = std::io::stdin();

    for form in forms {
        print!("{}: ", form.name);
        std::io::stdout().flush().unwrap();

        let replace_text = &mut String::new();
        stdin.read_line(replace_text).unwrap();

        let len = replace_text.len();
        replace_text.truncate(len - 1);

        text.fill_form(&form, replace_text);
    }

    let mut dst = File::create(dst_path).unwrap();
    dst.write(text.input.as_bytes()).unwrap();
}

fn main() {
    let matches = App::new("fill")
        .version(clap::crate_version!())
        .about("Just a file filler.")
        .arg(
            Arg::with_name("template")
                .required(true)
                .index(1)
                .help("Input template file"),
        )
        .arg(
            Arg::with_name("destination")
                .required(true)
                .index(2)
                .help("Save location of the result"),
        )
        .get_matches();

    fill(
        matches.value_of("template").unwrap(),
        matches.value_of("destination").unwrap(),
    );
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
