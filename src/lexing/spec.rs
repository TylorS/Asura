use regex::Regex;

use super::token::{Position, Token, TokenKind};

pub struct Spec<'a> {
    pub spec: Vec<SpecFn<'a>>,
}

pub type SpecFn<'a> = fn(&'a str, usize) -> Option<Token<'a>>;

impl<'a> Spec<'a> {
    pub fn new(spec: Vec<SpecFn<'a>>) -> Spec<'a> {
        Spec { spec }
    }

    pub fn asura() -> Spec<'a> {
        Spec::new(vec![
            // Literals
            Spec::boolean_literal,
            Spec::number_literal,
            Spec::regexp_literal,
            Spec::string_literal,
            Spec::template_literal,
            // Keywords
            Spec::alias,
            Spec::brand,
            Spec::case,
            Spec::effect,
            Spec::export,
            Spec::else_,
            Spec::else_if,
            Spec::for_,
            Spec::from,
            Spec::function,
            Spec::if_,
            Spec::import,
            Spec::macro_,
            Spec::match_,
            Spec::return_,
            Spec::struct_,
            Spec::type_,
            Spec::typeclass,
            Spec::while_,
            Spec::with,
            // Delimiters
            Spec::dot_dot_dot,
            Spec::dot_dot,
            Spec::dot,
            Spec::at,
            Spec::colon,
            Spec::comma,
            Spec::left_brace,
            Spec::left_bracket,
            Spec::left_paren,
            Spec::right_brace,
            Spec::right_bracket,
            Spec::right_paren,
            Spec::semicolon,
            Spec::underscore,
            Spec::pipe,
            Spec::left_arrow,
            Spec::fat_arrow,
            // Symbols + operators
            Spec::and_and,
            Spec::and,
            Spec::back_slash,
            Spec::bang_equal,
            Spec::bang,
            Spec::caret,
            Spec::dollar,
            Spec::equal_equal,
            Spec::equal,
            Spec::greater_than_greater_than_greater_than,
            Spec::greater_than_greater_than,
            Spec::greater_than_equal,
            Spec::greater_than,
            Spec::hash,
            Spec::less_than_less_than,
            Spec::less_than_equal,
            Spec::less_than,
            Spec::minus_minus,
            Spec::minus,
            Spec::or_or,
            Spec::or,
            Spec::percent,
            Spec::plus_plus,
            Spec::plus,
            Spec::question,
            Spec::slash,
            Spec::star_star,
            Spec::star,
            Spec::tilde,
            // Identifiers
            Spec::macro_identifier, // needs to come before identifier as they're idential, but macros have a ! at the end
            Spec::identifier,
            // Whitespace
            Spec::whitespace,
            // Comments
            Spec::comment,
            Spec::doc_comment,
        ])
    }

    fn boolean_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(true|false)").unwrap(),
            TokenKind::BooleanLiteral,
        )
    }

    fn number_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\d)+").unwrap(),
            TokenKind::NumberLiteral,
        )
    }

    fn regexp_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(/.+/)").unwrap(),
            TokenKind::RegexpLiteral,
        )
    }

    fn string_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r#"^["'](.+)['"]"#).unwrap(),
            TokenKind::StringLiteral,
        )
    }

    fn template_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r#"^(`.*`)"#).unwrap(),
            TokenKind::TemplateLiteral,
        )
    }

    fn identifier(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)").unwrap(),
            TokenKind::Identifier,
        )
    }

    fn macro_identifier(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*!)").unwrap(),
            TokenKind::MacroIdentifier,
        )
    }

    fn alias(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(alias)").unwrap(),
            TokenKind::Alias,
        )
    }

    fn brand(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(brand)").unwrap(),
            TokenKind::Brand,
        )
    }

    fn case(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(case)").unwrap(),
            TokenKind::Case,
        )
    }

    fn effect(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(effect)").unwrap(),
            TokenKind::Effect,
        )
    }

    fn export(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(export)").unwrap(),
            TokenKind::Export,
        )
    }

    fn else_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(else)").unwrap(),
            TokenKind::Else,
        )
    }

    fn else_if(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(else if)").unwrap(),
            TokenKind::ElseIf,
        )
    }

    fn for_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(for)").unwrap(),
            TokenKind::For,
        )
    }

    fn from(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(from)").unwrap(),
            TokenKind::From,
        )
    }

    fn function(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(fun)").unwrap(),
            TokenKind::Function,
        )
    }

    fn if_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(if)").unwrap(),
            TokenKind::If,
        )
    }

    fn import(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(import)").unwrap(),
            TokenKind::Import,
        )
    }

    fn macro_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(macro)").unwrap(),
            TokenKind::Macro,
        )
    }

    fn match_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(match)").unwrap(),
            TokenKind::Match,
        )
    }

    fn return_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(return)").unwrap(),
            TokenKind::Return,
        )
    }

    fn struct_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(struct)").unwrap(),
            TokenKind::Struct,
        )
    }

    fn type_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(type)").unwrap(),
            TokenKind::Type,
        )
    }

    fn typeclass(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(typeclass)").unwrap(),
            TokenKind::Typeclass,
        )
    }

    fn while_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(while)").unwrap(),
            TokenKind::While,
        )
    }

    fn with(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(with)").unwrap(),
            TokenKind::With,
        )
    }

    fn and(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(&)").unwrap(),
            TokenKind::And,
        )
    }

    fn and_and(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(&&)").unwrap(),
            TokenKind::AndAnd,
        )
    }

    fn back_slash(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\\)").unwrap(),
            TokenKind::Backslash,
        )
    }

    fn bang(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(!)").unwrap(),
            TokenKind::Bang,
        )
    }

    fn bang_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(!=)").unwrap(),
            TokenKind::BangEqual,
        )
    }

    fn caret(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\^)").unwrap(),
            TokenKind::Caret,
        )
    }

    fn dollar(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\$)").unwrap(),
            TokenKind::Dollar,
        )
    }

    fn equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(=)").unwrap(),
            TokenKind::Equal,
        )
    }

    fn equal_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(==)").unwrap(),
            TokenKind::EqualEqual,
        )
    }

    fn greater_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(>)").unwrap(),
            TokenKind::GreaterThan,
        )
    }

    fn greater_than_greater_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(>>)").unwrap(),
            TokenKind::GreaterThanGreaterThan,
        )
    }

    fn greater_than_greater_than_greater_than(
        input: &'a str,
        position: usize,
    ) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(>>>)").unwrap(),
            TokenKind::GreaterThanGreaterThanGreaterThan,
        )
    }

    fn greater_than_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(>=)").unwrap(),
            TokenKind::GreaterThanEqual,
        )
    }

    fn hash(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\#)").unwrap(),
            TokenKind::Hash,
        )
    }

    fn less_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(<)").unwrap(),
            TokenKind::LessThan,
        )
    }

    fn less_than_less_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(<<)").unwrap(),
            TokenKind::LessThanLessThan,
        )
    }

    fn less_than_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(<=)").unwrap(),
            TokenKind::LessThanEqual,
        )
    }

    fn minus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(-)").unwrap(),
            TokenKind::Minus,
        )
    }

    fn minus_minus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\-\-)").unwrap(),
            TokenKind::MinusMinus,
        )
    }

    fn or(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\|)").unwrap(),
            TokenKind::Or,
        )
    }

    fn or_or(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\|\|)").unwrap(),
            TokenKind::OrOr,
        )
    }

    fn percent(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\%)").unwrap(),
            TokenKind::Percent,
        )
    }

    fn plus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\+)").unwrap(),
            TokenKind::Plus,
        )
    }

    fn plus_plus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\+\+)").unwrap(),
            TokenKind::PlusPlus,
        )
    }

    fn question(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\?)").unwrap(),
            TokenKind::Question,
        )
    }

    fn slash(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(/)").unwrap(),
            TokenKind::Slash,
        )
    }

    fn star(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\*)").unwrap(),
            TokenKind::Star,
        )
    }

    fn star_star(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\*\*)").unwrap(),
            TokenKind::StarStar,
        )
    }

    fn tilde(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\~)").unwrap(),
            TokenKind::Tilde,
        )
    }

    fn at(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\@)").unwrap(),
            TokenKind::At,
        )
    }

    fn colon(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(:)").unwrap(),
            TokenKind::Colon,
        )
    }

    fn comma(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\,)").unwrap(),
            TokenKind::Comma,
        )
    }

    fn dot(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\.)").unwrap(),
            TokenKind::Dot,
        )
    }

    fn dot_dot(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\.\.)").unwrap(),
            TokenKind::DotDot,
        )
    }

    fn dot_dot_dot(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\.\.\.)").unwrap(),
            TokenKind::DotDotDot,
        )
    }

    fn fat_arrow(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(=>)").unwrap(),
            TokenKind::FatArrow,
        )
    }

    fn left_arrow(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(<\-)").unwrap(),
            TokenKind::LeftArrow,
        )
    }

    fn left_brace(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\{)").unwrap(),
            TokenKind::LeftBrace,
        )
    }

    fn left_bracket(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\[)").unwrap(),
            TokenKind::LeftBracket,
        )
    }

    fn left_paren(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\()").unwrap(),
            TokenKind::LeftParen,
        )
    }

    fn pipe(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\|>)").unwrap(),
            TokenKind::Pipe,
        )
    }

    fn right_brace(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\})").unwrap(),
            TokenKind::RightBrace,
        )
    }

    fn right_bracket(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\])").unwrap(),
            TokenKind::RightBracket,
        )
    }

    fn right_paren(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\))").unwrap(),
            TokenKind::RightParen,
        )
    }

    fn semicolon(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\;)").unwrap(),
            TokenKind::Semicolon,
        )
    }

    fn underscore(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(\_)").unwrap(),
            TokenKind::Underscore,
        )
    }

    // Comments are started with two slashes and end with a newline
    fn comment(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(//).*\n").unwrap(),
            TokenKind::Comment,
        )
    }

    fn doc_comment(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^(/\*\*).*\*/").unwrap(),
            TokenKind::DocComment,
        )
    }

    fn whitespace(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            Regex::new(r"^([ \t\r\n])+").unwrap(),
            TokenKind::WhiteSpace,
        )
    }
}

fn find_regex(
    input: &str,
    position: usize,
    re: Regex,
    kind: TokenKind,
) -> Option<Token<'_>> {
    re.find(input).map(|m| {
        let pos = Position::new(position + m.start(), position + m.end());

        Token::new(kind, m.as_str(), pos)
    })
}
