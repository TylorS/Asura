mod lexing;
use lexing::*;

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
        let fib_asura: String = read_to_string("examples/fib.asura")?;

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
            Token::new(TokenKind::Identifier, "n", Position::new(99, 100)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(100, 101)),
            Token::new(TokenKind::Minus, "-", Position::new(101, 102)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(102, 103)),
            Token::new(TokenKind::NumberLiteral, "2", Position::new(103, 104)),
            Token::new(TokenKind::RightParen, ")", Position::new(104, 105)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(105, 106)),
            Token::new(TokenKind::Plus, "+", Position::new(106, 107)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(107, 108)),
            Token::new(TokenKind::Identifier, "fib", Position::new(108, 111)),
            Token::new(TokenKind::LeftParen, "(", Position::new(111, 112)),
            Token::new(TokenKind::Identifier, "n", Position::new(112, 113)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(113, 114)),
            Token::new(TokenKind::Minus, "-", Position::new(114, 115)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(115, 116)),
            Token::new(TokenKind::NumberLiteral, "1", Position::new(116, 117)),
            Token::new(TokenKind::RightParen, ")", Position::new(117, 118)),
            Token::new(TokenKind::Comma, ",", Position::new(118, 119)),
            Token::new(TokenKind::WhiteSpace, "\n  ", Position::new(119, 122)),
            Token::new(TokenKind::RightBrace, "}", Position::new(122, 123)),
            Token::new(TokenKind::WhiteSpace, "\n", Position::new(123, 124)),
            Token::new(TokenKind::RightBrace, "}", Position::new(124, 125)),
            Token::new(TokenKind::WhiteSpace, "\n\n", Position::new(125, 127)),
            Token::new(TokenKind::Function, "fun", Position::new(127, 130)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(130, 131)),
            Token::new(TokenKind::Identifier, "main", Position::new(131, 135)),
            Token::new(TokenKind::LeftParen, "(", Position::new(135, 136)),
            Token::new(TokenKind::Identifier, "n", Position::new(136, 137)),
            Token::new(TokenKind::Colon, ":", Position::new(137, 138)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(138, 139)),
            Token::new(TokenKind::Identifier, "Int", Position::new(139, 142)),
            Token::new(TokenKind::RightParen, ")", Position::new(142, 143)),
            Token::new(TokenKind::Colon, ":", Position::new(143, 144)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(144, 145)),
            Token::new(TokenKind::Identifier, "Effect", Position::new(145, 151)),
            Token::new(TokenKind::LessThan, "<", Position::new(151, 152)),
            Token::new(TokenKind::Identifier, "Console", Position::new(152, 159)),
            Token::new(TokenKind::Comma, ",", Position::new(159, 160)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(160, 161)),
            Token::new(TokenKind::Identifier, "Unit", Position::new(161, 165)),
            Token::new(TokenKind::GreaterThan, ">", Position::new(165, 166)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(166, 167)),
            Token::new(TokenKind::LeftBrace, "{", Position::new(167, 168)),
            Token::new(TokenKind::WhiteSpace, "\n  ", Position::new(168, 171)),
            Token::new(TokenKind::Underscore, "_", Position::new(171, 172)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(172, 173)),
            Token::new(TokenKind::LeftArrow, "<-", Position::new(173, 175)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(175, 176)),
            Token::new(TokenKind::Identifier, "Console", Position::new(176, 183)),
            Token::new(TokenKind::Dot, ".", Position::new(183, 184)),
            Token::new(TokenKind::Identifier, "log", Position::new(184, 187)),
            Token::new(TokenKind::LeftParen, "(", Position::new(187, 188)),
            Token::new(
                TokenKind::TemplateLiteral,
                "`Calculating fib(${n})...`",
                Position::new(188, 214),
            ),
            Token::new(TokenKind::RightParen, ")", Position::new(214, 215)),
            Token::new(TokenKind::WhiteSpace, "\n  ", Position::new(215, 218)),
            Token::new(TokenKind::Underscore, "_", Position::new(218, 219)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(219, 220)),
            Token::new(TokenKind::LeftArrow, "<-", Position::new(220, 222)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(222, 223)),
            Token::new(TokenKind::Identifier, "Console", Position::new(223, 230)),
            Token::new(TokenKind::Dot, ".", Position::new(230, 231)),
            Token::new(TokenKind::Identifier, "log", Position::new(231, 234)),
            Token::new(TokenKind::LeftParen, "(", Position::new(234, 235)),
            Token::new(
                TokenKind::TemplateLiteral,
                "`Calculated fib(${n}): ${fib(n)}.`",
                Position::new(235, 269),
            ),
            Token::new(TokenKind::RightParen, ")", Position::new(269, 270)),
            Token::new(TokenKind::WhiteSpace, "\n", Position::new(270, 271)),
            Token::new(TokenKind::RightBrace, "}", Position::new(271, 272)),
            Token::new(TokenKind::WhiteSpace, "\n\n", Position::new(272, 274)),
            Token::new(TokenKind::Identifier, "main", Position::new(274, 278)),
            Token::new(TokenKind::LeftParen, "(", Position::new(278, 279)),
            Token::new(TokenKind::NumberLiteral, "10", Position::new(279, 281)),
            Token::new(TokenKind::RightParen, ")", Position::new(281, 282)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(282, 283)),
            Token::new(TokenKind::With, "with", Position::new(283, 287)),
            Token::new(TokenKind::WhiteSpace, " ", Position::new(287, 288)),
            Token::new(TokenKind::Identifier, "Console", Position::new(288, 295)),
            Token::new(TokenKind::Dot, ".", Position::new(295, 296)),
            Token::new(TokenKind::Identifier, "Platform", Position::new(296, 304)),
            Token::new(TokenKind::WhiteSpace, "\n", Position::new(304, 305)),
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
