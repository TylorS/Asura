mod tokenizer;

pub fn tokenize(input: &str) -> Vec<tokenizer::token::Token> {
    let mut tokenizer = tokenizer::tokenizer::Tokenizer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }

    tokens
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
