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
            Token::import("import", Position::new(0, 6)),
            Token::whitespace(" ", Position::new(6, 7)),
            Token::identifier("Console", Position::new(7, 14)),
            Token::whitespace(" ", Position::new(14, 15)),
            Token::from("from", Position::new(15, 19)),
            Token::whitespace(" ", Position::new(19, 20)),
            Token::string_literal("'std:Console'", Position::new(20, 33)),
            Token::whitespace("\n\n", Position::new(33, 35)),
            Token::function("fun", Position::new(35, 38)),
            Token::whitespace(" ", Position::new(38, 39)),
            Token::identifier("fib", Position::new(39, 42)),
            Token::left_paren("(", Position::new(42, 43)),
            Token::identifier("n", Position::new(43, 44)),
            Token::colon(":", Position::new(44, 45)),
            Token::whitespace(" ", Position::new(45, 46)),
            Token::identifier("Int", Position::new(46, 49)),
            Token::right_paren(")", Position::new(49, 50)),
            Token::colon(":", Position::new(50, 51)),
            Token::whitespace(" ", Position::new(51, 52)),
            Token::identifier("Int", Position::new(52, 55)),
            Token::whitespace(" ", Position::new(55, 56)),
            Token::left_brace("{", Position::new(56, 57)),
            Token::whitespace("\n  ", Position::new(57, 60)),
            Token::match_("match", Position::new(60, 65)),
            Token::whitespace(" ", Position::new(65, 66)),
            Token::identifier("n", Position::new(66, 67)),
            Token::whitespace(" ", Position::new(67, 68)),
            Token::left_brace("{", Position::new(68, 69)),
            Token::whitespace("\n    ", Position::new(69, 74)),
            Token::identifier("n", Position::new(74, 75)),
            Token::whitespace(" ", Position::new(75, 76)),
            Token::less_than("<", Position::new(76, 77)),
            Token::whitespace(" ", Position::new(77, 78)),
            Token::number_literal("2", Position::new(78, 79)),
            Token::whitespace(" ", Position::new(79, 80)),
            Token::fat_arrow("=>", Position::new(80, 82)),
            Token::whitespace(" ", Position::new(82, 83)),
            Token::identifier("n", Position::new(83, 84)),
            Token::comma(",", Position::new(84, 85)),
            Token::whitespace("\n    ", Position::new(85, 90)),
            Token::underscore("_", Position::new(90, 91)),
            Token::whitespace(" ", Position::new(91, 92)),
            Token::fat_arrow("=>", Position::new(92, 94)),
            Token::whitespace(" ", Position::new(94, 95)),
            Token::identifier("fib", Position::new(95, 98)),
            Token::left_paren("(", Position::new(98, 99)),
            Token::identifier("n", Position::new(99, 100)),
            Token::whitespace(" ", Position::new(100, 101)),
            Token::minus("-", Position::new(101, 102)),
            Token::whitespace(" ", Position::new(102, 103)),
            Token::number_literal("2", Position::new(103, 104)),
            Token::right_paren(")", Position::new(104, 105)),
            Token::whitespace(" ", Position::new(105, 106)),
            Token::plus("+", Position::new(106, 107)),
            Token::whitespace(" ", Position::new(107, 108)),
            Token::identifier("fib", Position::new(108, 111)),
            Token::left_paren("(", Position::new(111, 112)),
            Token::identifier("n", Position::new(112, 113)),
            Token::whitespace(" ", Position::new(113, 114)),
            Token::minus("-", Position::new(114, 115)),
            Token::whitespace(" ", Position::new(115, 116)),
            Token::number_literal("1", Position::new(116, 117)),
            Token::right_paren(")", Position::new(117, 118)),
            Token::comma(",", Position::new(118, 119)),
            Token::whitespace("\n  ", Position::new(119, 122)),
            Token::right_brace("}", Position::new(122, 123)),
            Token::whitespace("\n", Position::new(123, 124)),
            Token::right_brace("}", Position::new(124, 125)),
            Token::whitespace("\n\n", Position::new(125, 127)),
            Token::function("fun", Position::new(127, 130)),
            Token::whitespace(" ", Position::new(130, 131)),
            Token::identifier("main", Position::new(131, 135)),
            Token::left_paren("(", Position::new(135, 136)),
            Token::identifier("n", Position::new(136, 137)),
            Token::colon(":", Position::new(137, 138)),
            Token::whitespace(" ", Position::new(138, 139)),
            Token::identifier("Int", Position::new(139, 142)),
            Token::right_paren(")", Position::new(142, 143)),
            Token::colon(":", Position::new(143, 144)),
            Token::whitespace(" ", Position::new(144, 145)),
            Token::identifier("Effect", Position::new(145, 151)),
            Token::less_than("<", Position::new(151, 152)),
            Token::identifier("Console", Position::new(152, 159)),
            Token::comma(",", Position::new(159, 160)),
            Token::whitespace(" ", Position::new(160, 161)),
            Token::identifier("Unit", Position::new(161, 165)),
            Token::greater_than(">", Position::new(165, 166)),
            Token::whitespace(" ", Position::new(166, 167)),
            Token::left_brace("{", Position::new(167, 168)),
            Token::whitespace("\n  ", Position::new(168, 171)),
            Token::underscore("_", Position::new(171, 172)),
            Token::whitespace(" ", Position::new(172, 173)),
            Token::left_arrow("<-", Position::new(173, 175)),
            Token::whitespace(" ", Position::new(175, 176)),
            Token::identifier("Console", Position::new(176, 183)),
            Token::dot(".", Position::new(183, 184)),
            Token::identifier("log", Position::new(184, 187)),
            Token::left_paren("(", Position::new(187, 188)),
            Token::template_literal("`Calculating fib(${n})...`", Position::new(188, 214)),
            Token::right_paren(")", Position::new(214, 215)),
            Token::whitespace("\n  ", Position::new(215, 218)),
            Token::underscore("_", Position::new(218, 219)),
            Token::whitespace(" ", Position::new(219, 220)),
            Token::left_arrow("<-", Position::new(220, 222)),
            Token::whitespace(" ", Position::new(222, 223)),
            Token::identifier("Console", Position::new(223, 230)),
            Token::dot(".", Position::new(230, 231)),
            Token::identifier("log", Position::new(231, 234)),
            Token::left_paren("(", Position::new(234, 235)),
            Token::template_literal(
                "`Calculated fib(${n}): ${fib(n)}.`",
                Position::new(235, 269),
            ),
            Token::right_paren(")", Position::new(269, 270)),
            Token::whitespace("\n", Position::new(270, 271)),
            Token::right_brace("}", Position::new(271, 272)),
            Token::whitespace("\n\n", Position::new(272, 274)),
            Token::identifier("main", Position::new(274, 278)),
            Token::left_paren("(", Position::new(278, 279)),
            Token::number_literal("10", Position::new(279, 281)),
            Token::right_paren(")", Position::new(281, 282)),
            Token::whitespace(" ", Position::new(282, 283)),
            Token::with("with", Position::new(283, 287)),
            Token::whitespace(" ", Position::new(287, 288)),
            Token::identifier("Console", Position::new(288, 295)),
            Token::dot(".", Position::new(295, 296)),
            Token::identifier("Platform", Position::new(296, 304)),
            Token::whitespace("\n", Position::new(304, 305)),
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
