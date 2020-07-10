use std::fmt;
use std::rc::Rc;
use std::vec::Vec;

pub enum Token {
    Invalid,
    Literal(String),
    Pattern(String),
    // TODO support?
    Regex(String),
}

pub struct PathError {
    message: String,
}

impl PathError {
    pub fn new(message: String) -> PathError {
        return PathError { message };
    }
}

impl fmt::Display for PathError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "{}", self.message);
    }
}

impl fmt::Debug for PathError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "PathError {{ message: {} }}", self.message,);
    }
}

pub struct Path {
    literal: String,
    tokens: Vec<Token>,
}

impl Path {
    // TODO
    fn validate(literal: &str) -> bool {
        return false;
    }

    pub fn new() -> Path {
        return Path {
            literal: String::from(""),
            tokens: Vec::new(),
        };
    }

    // TODO the string has to be decoded from URL encoded form before returning a token
    pub fn parse(literal: String) -> Result<Path, PathError> {
        // TODO support anything other than literals
        let tokens: Vec<_> = literal
            .split("/")
            .map(|tok| Token::Literal(String::from(tok)))
            .collect();
        return Ok(Path { literal, tokens });

        /*
         *        for tok in tokens_iter {
         *            // The minimum a non literal can be is 4 since <r:tok> or <p:tok>
         *            if tok.len() < 5 {
         *                tok_vec.append(Token::Literal(tok));
         *            }
         *
         *            // URL validation
         *            if !Path::validate(tok) {
         *                return PathError::new("Invalid path");
         *            }
         *
         *            let tok: String = tok.split(|c: char| -> bool {
         *                return c == '<' || c == '>';
         *            }).collect::<String>();
         *        }
         */
    }
}
