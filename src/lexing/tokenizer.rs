use std::str::CharIndices;

use super::token::{Position, Token};

pub struct Tokenizer<'a> {
    input: CharIndices<'a>,
    length: usize,
    position: usize,
}

impl Tokenizer<'_> {
    pub fn new(input: &'_ str) -> Tokenizer<'_> {
        Tokenizer {
            input: input.char_indices(),
            length: input.len(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let char = self.next_char()?;

        match char {
            ' ' | '\t' | '\n' | '\r' => self.tokenize_whitespace(char),
            _ => None,
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        if self.position >= self.length {
            return None;
        }

        let (_, current_char) = self.input.next()?;
        self.position += 1;
        Some(current_char)
    }

    fn tokenize_whitespace(&mut self, initial: char) -> Option<Token> {
        let mut value: String = initial.into();

        let start = self.position - 1;
        let mut end = self.position;

        let mut add_char = |char: char| {
            value.push(char);
            end += 1;
        };

        while let Some(char) = self.next_char() {
            match char {
                ' ' | '\t' | '\n' | '\r' => add_char(char),
                _ => {
                    break;
                }
            }
        }

        if value.is_empty() {
            return None;
        }

        Some(Token::WhiteSpace(value, Position { start, end }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_traverses_characters() {
        let mut tokenizer = Tokenizer::new("hello");
        assert_eq!(tokenizer.next_char(), Some('h'));
        assert_eq!(tokenizer.next_char(), Some('e'));
        assert_eq!(tokenizer.next_char(), Some('l'));
        assert_eq!(tokenizer.next_char(), Some('l'));
        assert_eq!(tokenizer.next_char(), Some('o'));
    }
}
