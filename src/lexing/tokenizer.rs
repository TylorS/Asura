use super::{spec::Spec, token::Token};

pub struct Tokenizer<'a> {
    spec: Spec<'a>,
    input: &'a str,
    length: usize,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(spec: Spec<'a>) -> Tokenizer<'a> {
        Tokenizer {
            spec,
            input: "",
            length: 0,
            position: 0,
        }
    }

    pub fn asura() -> Tokenizer<'a> {
        Tokenizer::new(Spec::asura())
    }

    pub fn init(&mut self, input: &'a str) {
        self.input = input;
        self.length = input.len();
        self.position = 0;
    }

    // TODO: Eventually we want to get rid of the use of Spec and just use
    //       the Tokenizer to tokenize the input. This will require a bit of
    //       refactoring, but it will be worth it in terms of performance which
    //       is still quite good already.
    pub fn next_token(&mut self) -> Option<Token<'a>> {
        if self.position >= self.length {
            return None;
        }

        let input = &self.input[self.position..];

        for spec_fn in &self.spec.spec {
            if let Some(token) = spec_fn(input, self.position) {
                self.position = token.position().end;
                return Some(token);
            }
        }

        None
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
