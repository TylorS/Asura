mod lexing;
use lexing::*;

mod parsing;
pub use parsing::*;

pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut tokenizer = Tokenizer::asura();

    tokenizer.init(input);
    tokenizer.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, fs::read_to_string};

    #[test]
    fn it_tokenizes_whitespace() {
        let input = "     ";
        let result = tokenize(input);

        let exepected = vec![Token::whitespace(input, Position::new(0, 5))];

        assert_eq!(result.len(), 1);
        assert_eq!(result, exepected);
    }

    #[test]
    fn it_tokenizes_fib_example() -> Result<(), Box<dyn Error>> {
        let contents: String = read_to_string("examples/fib.asura")?;

        let result = tokenize(&contents);

        print_vector(result);

        Ok(())
    }

    #[test]
    fn it_tokenizes_state_example() -> Result<(), Box<dyn Error>> {
        let contents: String = read_to_string("examples/state.asura")?;

        let result = tokenize(&contents);

        print_vector(result);

        Ok(())
    }

    #[test]
    fn it_tokenizes_queue_example() -> Result<(), Box<dyn Error>> {
        let contents: String = read_to_string("examples/queue.asura")?;

        let result = tokenize(&contents);

        print_vector(result);

        Ok(())
    }

    fn print_vector<A>(vec: Vec<A>)
    where
        A: std::fmt::Debug,
    {
        println!("[");

        for item in vec {
            println!("    {:?},", item);
        }

        println!("]");
    }
}
