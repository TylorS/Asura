mod lexing;
use lexing::*;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::asura();

    tokenizer.init(input);

    return tokenizer.collect();
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
        let fib_asura: String = read_to_string("examples/fib.asura")?.into();

        let result = tokenize(&fib_asura);

        let expected = vec![
            Token::new(TokenKind::Import, "import", Position::new(0, 6)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(6, 7)),
            Token::new(TokenKind::Identifier, "Console", Position::new(7, 14)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(14, 15)),
            Token::new(TokenKind::From, "from", Position::new(15, 19)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(19, 20)),
            Token::new(
                TokenKind::StringLiteral,
                "'std:Console'",
                Position::new(20, 33),
            ),
            Token::new(TokenKind::WhiteSpace, "\n\n", Position::new(33, 35)),
            Token::new(TokenKind::Function, "fun", Position::new(35, 38)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(38, 39)),
            Token::new(TokenKind::Identifier, "fib", Position::new(39, 42)),
            Token::new(TokenKind::LeftParen, "(", Position::new(42, 43)),
            Token::new(TokenKind::Identifier, "n", Position::new(43, 44)),
            Token::new(TokenKind::Colon, ":", Position::new(44, 45)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(45, 46)),
            Token::new(TokenKind::Identifier, "Int", Position::new(46, 49)),
            Token::new(TokenKind::RightParen, ")", Position::new(49, 50)),
            Token::new(TokenKind::Colon, ":", Position::new(50, 51)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(51, 52)),
            Token::new(TokenKind::Identifier, "Int", Position::new(52, 55)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(55, 56)),
            Token::new(TokenKind::LeftBrace, "{", Position::new(56, 57)),
            Token::new(TokenKind::WhiteSpace, "\n  ", Position::new(57, 60)),
            Token::new(TokenKind::Match, "match", Position::new(60, 65)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(65, 66)),
            Token::new(TokenKind::Identifier, "n", Position::new(66, 67)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(67, 68)),
            Token::new(TokenKind::LeftBrace, "{", Position::new(68, 69)),
            Token::new(TokenKind::WhiteSpace, "\n    ", Position::new(69, 74)),
            Token::new(TokenKind::Identifier, "n", Position::new(74, 75)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(75, 76)),
            Token::new(TokenKind::LessThan, "<", Position::new(76, 77)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(77, 78)),
            Token::new(TokenKind::NumberLiteral, "2", Position::new(78, 79)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(79, 80)),
            Token::new(TokenKind::FatArrow, "=>", Position::new(80, 82)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(82, 83)),
            Token::new(TokenKind::Identifier, "n", Position::new(83, 84)),
            Token::new(TokenKind::Comma, ",", Position::new(84, 85)),
            Token::new(TokenKind::WhiteSpace, "\n    ", Position::new(85, 90)),
            Token::new(TokenKind::Underscore, "_", Position::new(90, 91)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(91, 92)),
            Token::new(TokenKind::FatArrow, "=>", Position::new(92, 94)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(94, 95)),
            Token::new(TokenKind::Identifier, "fib", Position::new(95, 98)),
            Token::new(TokenKind::LeftParen, "(", Position::new(98, 99)),
            Token::new(
                TokenKind::Identifier,
                "n",
                Position {
                    start: 99,
                    end: 100,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 100,
                    end: 101,
                },
            ),
            Token::new(
                TokenKind::Minus,
                "-",
                Position {
                    start: 101,
                    end: 102,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 102,
                    end: 103,
                },
            ),
            Token::new(
                TokenKind::NumberLiteral,
                "2",
                Position {
                    start: 103,
                    end: 104,
                },
            ),
            Token::new(
                TokenKind::RightParen,
                ")",
                Position {
                    start: 104,
                    end: 105,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 105,
                    end: 106,
                },
            ),
            Token::new(
                TokenKind::Plus,
                "+",
                Position {
                    start: 106,
                    end: 107,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 107,
                    end: 108,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "fib",
                Position {
                    start: 108,
                    end: 111,
                },
            ),
            Token::new(
                TokenKind::LeftParen,
                "(",
                Position {
                    start: 111,
                    end: 112,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "n",
                Position {
                    start: 112,
                    end: 113,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 113,
                    end: 114,
                },
            ),
            Token::new(
                TokenKind::Minus,
                "-",
                Position {
                    start: 114,
                    end: 115,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 115,
                    end: 116,
                },
            ),
            Token::new(
                TokenKind::NumberLiteral,
                "1",
                Position {
                    start: 116,
                    end: 117,
                },
            ),
            Token::new(
                TokenKind::RightParen,
                ")",
                Position {
                    start: 117,
                    end: 118,
                },
            ),
            Token::new(
                TokenKind::Comma,
                ",",
                Position {
                    start: 118,
                    end: 119,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n  ",
                Position {
                    start: 119,
                    end: 122,
                },
            ),
            Token::new(
                TokenKind::RightBrace,
                "}",
                Position {
                    start: 122,
                    end: 123,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n",
                Position {
                    start: 123,
                    end: 124,
                },
            ),
            Token::new(
                TokenKind::RightBrace,
                "}",
                Position {
                    start: 124,
                    end: 125,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n\n",
                Position {
                    start: 125,
                    end: 127,
                },
            ),
            Token::new(
                TokenKind::Function,
                "fun",
                Position {
                    start: 127,
                    end: 130,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 130,
                    end: 131,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "main",
                Position {
                    start: 131,
                    end: 135,
                },
            ),
            Token::new(
                TokenKind::LeftParen,
                "(",
                Position {
                    start: 135,
                    end: 136,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "n",
                Position {
                    start: 136,
                    end: 137,
                },
            ),
            Token::new(
                TokenKind::Colon,
                ":",
                Position {
                    start: 137,
                    end: 138,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 138,
                    end: 139,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Int",
                Position {
                    start: 139,
                    end: 142,
                },
            ),
            Token::new(
                TokenKind::RightParen,
                ")",
                Position {
                    start: 142,
                    end: 143,
                },
            ),
            Token::new(
                TokenKind::Colon,
                ":",
                Position {
                    start: 143,
                    end: 144,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 144,
                    end: 145,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Effect",
                Position {
                    start: 145,
                    end: 151,
                },
            ),
            Token::new(
                TokenKind::LessThan,
                "<",
                Position {
                    start: 151,
                    end: 152,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Console",
                Position {
                    start: 152,
                    end: 159,
                },
            ),
            Token::new(
                TokenKind::Comma,
                ",",
                Position {
                    start: 159,
                    end: 160,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 160,
                    end: 161,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Unit",
                Position {
                    start: 161,
                    end: 165,
                },
            ),
            Token::new(
                TokenKind::GreaterThan,
                ">",
                Position {
                    start: 165,
                    end: 166,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 166,
                    end: 167,
                },
            ),
            Token::new(
                TokenKind::LeftBrace,
                "{",
                Position {
                    start: 167,
                    end: 168,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n  ",
                Position {
                    start: 168,
                    end: 171,
                },
            ),
            Token::new(
                TokenKind::Underscore,
                "_",
                Position {
                    start: 171,
                    end: 172,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 172,
                    end: 173,
                },
            ),
            Token::new(
                TokenKind::LeftArrow,
                "<-",
                Position {
                    start: 173,
                    end: 175,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 175,
                    end: 176,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Console",
                Position {
                    start: 176,
                    end: 183,
                },
            ),
            Token::new(
                TokenKind::Dot,
                ".",
                Position {
                    start: 183,
                    end: 184,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "log",
                Position {
                    start: 184,
                    end: 187,
                },
            ),
            Token::new(
                TokenKind::LeftParen,
                "(",
                Position {
                    start: 187,
                    end: 188,
                },
            ),
            Token::new(
                TokenKind::TemplateLiteral,
                "`Calculating fib(${n})...`",
                Position {
                    start: 188,
                    end: 214,
                },
            ),
            Token::new(
                TokenKind::RightParen,
                ")",
                Position {
                    start: 214,
                    end: 215,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n  ",
                Position {
                    start: 215,
                    end: 218,
                },
            ),
            Token::new(
                TokenKind::Underscore,
                "_",
                Position {
                    start: 218,
                    end: 219,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 219,
                    end: 220,
                },
            ),
            Token::new(
                TokenKind::LeftArrow,
                "<-",
                Position {
                    start: 220,
                    end: 222,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 222,
                    end: 223,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Console",
                Position {
                    start: 223,
                    end: 230,
                },
            ),
            Token::new(
                TokenKind::Dot,
                ".",
                Position {
                    start: 230,
                    end: 231,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "log",
                Position {
                    start: 231,
                    end: 234,
                },
            ),
            Token::new(
                TokenKind::LeftParen,
                "(",
                Position {
                    start: 234,
                    end: 235,
                },
            ),
            Token::new(
                TokenKind::TemplateLiteral,
                "`Calculated fib(${n}): ${fib(n)}.`",
                Position {
                    start: 235,
                    end: 269,
                },
            ),
            Token::new(
                TokenKind::RightParen,
                ")",
                Position {
                    start: 269,
                    end: 270,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n",
                Position {
                    start: 270,
                    end: 271,
                },
            ),
            Token::new(
                TokenKind::RightBrace,
                "}",
                Position {
                    start: 271,
                    end: 272,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n\n",
                Position {
                    start: 272,
                    end: 274,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "main",
                Position {
                    start: 274,
                    end: 278,
                },
            ),
            Token::new(
                TokenKind::LeftParen,
                "(",
                Position {
                    start: 278,
                    end: 279,
                },
            ),
            Token::new(
                TokenKind::NumberLiteral,
                "10",
                Position {
                    start: 279,
                    end: 281,
                },
            ),
            Token::new(
                TokenKind::RightParen,
                ")",
                Position {
                    start: 281,
                    end: 282,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 282,
                    end: 283,
                },
            ),
            Token::new(
                TokenKind::With,
                "with",
                Position {
                    start: 283,
                    end: 287,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                " ",
                Position {
                    start: 287,
                    end: 288,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Console",
                Position {
                    start: 288,
                    end: 295,
                },
            ),
            Token::new(
                TokenKind::Dot,
                ".",
                Position {
                    start: 295,
                    end: 296,
                },
            ),
            Token::new(
                TokenKind::Identifier,
                "Platform",
                Position {
                    start: 296,
                    end: 304,
                },
            ),
            Token::new(
                TokenKind::WhiteSpace,
                "\n",
                Position {
                    start: 304,
                    end: 305,
                },
            ),
        ];

        assert_eq!(result, expected);

        Ok(())
    }

    // fn print_vector<A>(vec: Vec<A>)
    // where
    //     A: std::fmt::Debug,
    // {
    //     println!("[");

    //     for item in vec {
    //         println!("    {:?},", item);
    //     }

    //     println!("]");
    // }
}
