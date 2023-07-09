mod tokenizer;
use tokenizer::*;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tokenizes_whitespace() {
        let input = "     ";
        let result = tokenize(input);

        let exepected = vec![Token::WhiteSpace(
            input.to_string(),
            Position { start: 0, end: 5 },
        )];

        assert_eq!(result.len(), 1);
        assert_eq!(result, exepected);
    }
}
