use std::vec::Vec;

enum Token {
    Literal(Lit),
    Regex(regex),
    Pattern(pattern),
}

pub struct Path {
    literal: String,
    tokens: Vec<Token>,
}

impl Path {
    fn new(literal: String) {
        return Path {
            literal,
            Vec::new(),
        };
    }
}
