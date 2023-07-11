use lazy_static::lazy_static;
use regex::Regex;

use super::token::{Position, Token, TokenKind};

pub struct Spec<'a> {
    pub spec: Vec<SpecFn<'a>>,
}

pub type SpecFn<'a> = fn(&'a str, usize) -> Option<Token<'a>>;

lazy_static! {
    // Literals
    pub static ref BOOLEAN_LITERAL_REGEX: Regex = Regex::new(r"^(true|false)").unwrap();
    pub static ref NUMBER_LITERAL_REGEX: Regex = Regex::new(r"^(\d+(\.\d+)?)").unwrap();
    pub static ref REGEXP_LITERAL_REGEX: Regex = Regex::new(r"^\/.+\/").unwrap();
    pub static ref STRING_LITERAL_REGEX: Regex = Regex::new(r#"^["'](.+)['"]"#).unwrap();
    pub static ref TEMPLATE_LITERAL_REGEX: Regex = Regex::new(r#"^`([^`\\]|\\.)*`"#).unwrap();

    // Identifiers
    pub static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    pub static ref MACRO_IDENTIFIER_REGEX: Regex = Regex::new(r"^@[a-zA-Z_][a-zA-Z0-9_]*!").unwrap();

    // Keywords
    pub static ref ALIAS_REGEX: Regex = Regex::new(r"^(alias)").unwrap();
    pub static ref BRAND_REGEX: Regex = Regex::new(r"^(brand)").unwrap();
    pub static ref EFFECT_REGEX: Regex = Regex::new(r"^(effect)").unwrap();
    pub static ref EXPORT_REGEX: Regex = Regex::new(r"^(export)").unwrap();
    pub static ref ELSE_REGEX: Regex = Regex::new(r"^(else)").unwrap();
    pub static ref ELSE_IF_REGEX: Regex = Regex::new(r"^(else if)").unwrap();
    pub static ref FOR_REGEX: Regex = Regex::new(r"^(for)").unwrap();
    pub static ref FROM_REGEX: Regex = Regex::new(r"^(from)").unwrap();
    pub static ref FUNCTION_REGEX: Regex = Regex::new(r"^(function)").unwrap();
    pub static ref HANDLER_REGEX: Regex = Regex::new(r"^(handler)").unwrap();
    pub static ref IF_REGEX: Regex = Regex::new(r"^(if)").unwrap();
    pub static ref IMPORT_REGEX: Regex = Regex::new(r"^(import)").unwrap();
    pub static ref MACRO_REGEX: Regex = Regex::new(r"^(macro)").unwrap();
    pub static ref MATCH_REGEX: Regex = Regex::new(r"^(match)").unwrap();
    pub static ref OF_REGEX: Regex = Regex::new(r"^(of)").unwrap();
    pub static ref RETURN_REGEX: Regex = Regex::new(r"^(return)").unwrap();
    pub static ref STRUCT_REGEX: Regex = Regex::new(r"^(struct)").unwrap();
    pub static ref TYPE_REGEX: Regex = Regex::new(r"^(type)").unwrap();
    pub static ref TYPECLASS_REGEX: Regex = Regex::new(r"^(typeclass)").unwrap();
    pub static ref WHILE_REGEX: Regex = Regex::new(r"^(while)").unwrap();
    pub static ref WITH_REGEX: Regex = Regex::new(r"^(with)").unwrap();
    pub static ref YIELD_REGEX: Regex = Regex::new(r"^(yield)").unwrap();

    // Symbols + Operators

    pub static ref AND_REGEX: Regex = Regex::new(r"^&").unwrap();
    pub static ref AND_AND_REGEX: Regex = Regex::new(r"^&&").unwrap();
    pub static ref BACKSLASH_REGEX: Regex = Regex::new(r"^\\").unwrap();
    pub static ref BANG_REGEX: Regex = Regex::new(r"^!").unwrap();
    pub static ref BANG_EQUAL_REGEX: Regex = Regex::new(r"^!=").unwrap();
    pub static ref CARET_REGEX: Regex = Regex::new(r"^\^").unwrap();
    pub static ref DOLLAR_REGEX: Regex = Regex::new(r"^\$").unwrap();
    pub static ref EQUAL_REGEX: Regex = Regex::new(r"^=").unwrap();
    pub static ref EQUAL_EQUAL_REGEX: Regex = Regex::new(r"^==").unwrap();
    pub static ref GREATER_THAN_REGEX: Regex = Regex::new(r"^>").unwrap();
    pub static ref GREATER_THAN_GREATER_THAN_REGEX: Regex = Regex::new(r"^>>").unwrap();
    pub static ref GREATER_THAN_GREATER_THAN_GREATER_THAN_REGEX: Regex = Regex::new(r"^>>>").unwrap();
    pub static ref GREATER_THAN_EQUAL_REGEX: Regex = Regex::new(r"^>=").unwrap();
    pub static ref HASH_REGEX: Regex = Regex::new(r"^#").unwrap();
    pub static ref LESS_THAN_REGEX: Regex = Regex::new(r"^<").unwrap();
    pub static ref LESS_THAN_LESS_THAN_REGEX: Regex = Regex::new(r"^<<").unwrap();
    pub static ref LESS_THAN_EQUAL_REGEX: Regex = Regex::new(r"^<=").unwrap();
    pub static ref MINUS_REGEX: Regex = Regex::new(r"^-").unwrap();
    pub static ref MINUS_MINUS_REGEX: Regex = Regex::new(r"^--").unwrap();
    pub static ref OR_REGEX: Regex = Regex::new(r"^\|").unwrap();
    pub static ref OR_OR_REGEX: Regex = Regex::new(r"^\|\|").unwrap();
    pub static ref PERCENT_REGEX: Regex = Regex::new(r"^%").unwrap();
    pub static ref PLUS_REGEX: Regex = Regex::new(r"^\+").unwrap();
    pub static ref PLUS_PLUS_REGEX: Regex = Regex::new(r"^\+\+").unwrap();
    pub static ref QUESTION_MARK_REGEX: Regex = Regex::new(r"^\?").unwrap();
    pub static ref SLASH_REGEX: Regex = Regex::new(r"^/").unwrap();
    pub static ref STAR_REGEX: Regex = Regex::new(r"^\*").unwrap();
    pub static ref STAR_STAR_REGEX: Regex = Regex::new(r"^\*\*").unwrap();
    pub static ref TILDE_REGEX: Regex = Regex::new(r"^~").unwrap();

    // Delimiters
    pub static ref AT_REGEX: Regex = Regex::new(r"^@").unwrap();
    pub static ref COLON_REGEX: Regex = Regex::new(r"^:").unwrap();
    pub static ref COLON_EQUAL_REGEX: Regex = Regex::new(r"^:=").unwrap();
    pub static ref COMMA_REGEX: Regex = Regex::new(r"^,").unwrap();
    pub static ref DOT_REGEX: Regex = Regex::new(r"^\.").unwrap();
    pub static ref DOT_DOT_REGEX: Regex = Regex::new(r"^\.\.").unwrap();
    pub static ref DOT_DOT_DOT_REGEX: Regex = Regex::new(r"^\.\.\.").unwrap();
    pub static ref FAT_ARROW_REGEX: Regex = Regex::new(r"^=>").unwrap();
    pub static ref LEFT_ARROW_REGEX: Regex = Regex::new(r"^<-").unwrap();
    pub static ref LEFT_BRACE_REGEX: Regex = Regex::new(r"^\{").unwrap();
    pub static ref LEFT_BRACKET_REGEX: Regex = Regex::new(r"^\[").unwrap();
    pub static ref LEFT_PAREN_REGEX: Regex = Regex::new(r"^\(").unwrap();
    pub static ref PIPE_REGEX: Regex = Regex::new(r"^\|>").unwrap();
    pub static ref RIGHT_ARROW_REGEX: Regex = Regex::new(r"^->").unwrap();
    pub static ref RIGHT_BRACE_REGEX: Regex = Regex::new(r"^\}").unwrap();
    pub static ref RIGHT_BRACKET_REGEX: Regex = Regex::new(r"^\]").unwrap();
    pub static ref RIGHT_PAREN_REGEX: Regex = Regex::new(r"^\)").unwrap();
    pub static ref SEMICOLON_REGEX: Regex = Regex::new(r"^;").unwrap();
    pub static ref UNDERSCORE_REGEX: Regex = Regex::new(r"^_").unwrap();

    // Whitespace
    pub static ref WHITESPACE_REGEX: Regex = Regex::new(r"^([ \t\r\n])+").unwrap();

    // Comments
    pub static ref COMMENT_REGEX: Regex = Regex::new(r"^//.*").unwrap();
    pub static ref DOC_COMMENT_REGEX: Regex = Regex::new(r"^(/\*\*).*\*/").unwrap();
}

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
            Spec::effect,
            Spec::export,
            Spec::else_,
            Spec::else_if,
            Spec::for_,
            Spec::from,
            Spec::function,
            Spec::handler,
            Spec::if_,
            Spec::import,
            Spec::macro_,
            Spec::match_,
            Spec::of,
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
            Spec::colon_equal,
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
            Spec::right_arrow,
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
            &BOOLEAN_LITERAL_REGEX,
            TokenKind::BooleanLiteral,
        )
    }

    fn number_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &NUMBER_LITERAL_REGEX,
            TokenKind::NumberLiteral,
        )
    }

    fn regexp_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &REGEXP_LITERAL_REGEX,
            TokenKind::RegexpLiteral,
        )
    }

    fn string_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &STRING_LITERAL_REGEX,
            TokenKind::StringLiteral,
        )
    }

    fn template_literal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &TEMPLATE_LITERAL_REGEX,
            TokenKind::TemplateLiteral,
        )
    }

    fn identifier(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &IDENTIFIER_REGEX, TokenKind::Identifier)
    }

    fn macro_identifier(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &MACRO_IDENTIFIER_REGEX,
            TokenKind::MacroIdentifier,
        )
    }

    fn alias(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &ALIAS_REGEX, TokenKind::Alias)
    }

    fn brand(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &BRAND_REGEX, TokenKind::Brand)
    }

    fn effect(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &EFFECT_REGEX, TokenKind::Effect)
    }

    fn export(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &EXPORT_REGEX, TokenKind::Export)
    }

    fn else_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &ELSE_REGEX, TokenKind::Else)
    }

    fn else_if(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &ELSE_IF_REGEX, TokenKind::ElseIf)
    }

    fn for_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &FOR_REGEX, TokenKind::For)
    }

    fn from(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &FROM_REGEX, TokenKind::From)
    }

    fn function(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &FUNCTION_REGEX, TokenKind::Function)
    }

    fn handler(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &HANDLER_REGEX, TokenKind::Handler)
    }

    fn if_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &IF_REGEX, TokenKind::If)
    }

    fn import(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &IMPORT_REGEX, TokenKind::Import)
    }

    fn macro_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &MACRO_REGEX, TokenKind::Macro)
    }

    fn match_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &MATCH_REGEX, TokenKind::Match)
    }

    fn of(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &OF_REGEX, TokenKind::Of)
    }

    fn return_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &RETURN_REGEX, TokenKind::Return)
    }

    fn struct_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &STRUCT_REGEX, TokenKind::Struct)
    }

    fn type_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &TYPE_REGEX, TokenKind::Type)
    }

    fn typeclass(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &TYPECLASS_REGEX, TokenKind::Typeclass)
    }

    fn while_(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &WHILE_REGEX, TokenKind::While)
    }

    fn with(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &WITH_REGEX, TokenKind::With)
    }

    fn and(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &AND_REGEX, TokenKind::And)
    }

    fn and_and(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &AND_AND_REGEX, TokenKind::AndAnd)
    }

    fn back_slash(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &BACKSLASH_REGEX, TokenKind::Backslash)
    }

    fn bang(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &BANG_REGEX, TokenKind::Bang)
    }

    fn bang_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &BANG_EQUAL_REGEX, TokenKind::BangEqual)
    }

    fn caret(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &CARET_REGEX, TokenKind::Caret)
    }

    fn dollar(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &DOLLAR_REGEX, TokenKind::Dollar)
    }

    fn equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &EQUAL_REGEX, TokenKind::Equal)
    }

    fn equal_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &EQUAL_EQUAL_REGEX, TokenKind::EqualEqual)
    }

    fn greater_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &GREATER_THAN_REGEX, TokenKind::GreaterThan)
    }

    fn greater_than_greater_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &GREATER_THAN_GREATER_THAN_REGEX,
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
            &GREATER_THAN_GREATER_THAN_GREATER_THAN_REGEX,
            TokenKind::GreaterThanGreaterThanGreaterThan,
        )
    }

    fn greater_than_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &GREATER_THAN_EQUAL_REGEX,
            TokenKind::GreaterThanEqual,
        )
    }

    fn hash(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &HASH_REGEX, TokenKind::Hash)
    }

    fn less_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &LESS_THAN_REGEX, TokenKind::LessThan)
    }

    fn less_than_less_than(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &LESS_THAN_LESS_THAN_REGEX,
            TokenKind::LessThanLessThan,
        )
    }

    fn less_than_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &LESS_THAN_EQUAL_REGEX,
            TokenKind::LessThanEqual,
        )
    }

    fn minus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &MINUS_REGEX, TokenKind::Minus)
    }

    fn minus_minus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &MINUS_MINUS_REGEX, TokenKind::MinusMinus)
    }

    fn or(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &OR_REGEX, TokenKind::Or)
    }

    fn or_or(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &OR_OR_REGEX, TokenKind::OrOr)
    }

    fn percent(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &PERCENT_REGEX, TokenKind::Percent)
    }

    fn plus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &PLUS_REGEX, TokenKind::Plus)
    }

    fn plus_plus(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &PLUS_PLUS_REGEX, TokenKind::PlusPlus)
    }

    fn question(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &QUESTION_MARK_REGEX, TokenKind::Question)
    }

    fn slash(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &SLASH_REGEX, TokenKind::Slash)
    }

    fn star(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &STAR_REGEX, TokenKind::Star)
    }

    fn star_star(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &STAR_STAR_REGEX, TokenKind::StarStar)
    }

    fn tilde(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &TILDE_REGEX, TokenKind::Tilde)
    }

    fn at(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &AT_REGEX, TokenKind::At)
    }

    fn colon(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &COLON_REGEX, TokenKind::Colon)
    }

    fn colon_equal(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &COLON_EQUAL_REGEX, TokenKind::ColonEqual)
    }

    fn comma(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &COMMA_REGEX, TokenKind::Comma)
    }

    fn dot(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &DOT_REGEX, TokenKind::Dot)
    }

    fn dot_dot(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &DOT_DOT_REGEX, TokenKind::DotDot)
    }

    fn dot_dot_dot(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &DOT_DOT_DOT_REGEX, TokenKind::DotDotDot)
    }

    fn fat_arrow(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &FAT_ARROW_REGEX, TokenKind::FatArrow)
    }

    fn left_arrow(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &LEFT_ARROW_REGEX, TokenKind::LeftArrow)
    }

    fn left_brace(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &LEFT_BRACE_REGEX, TokenKind::LeftBrace)
    }

    fn left_bracket(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &LEFT_BRACKET_REGEX, TokenKind::LeftBracket)
    }

    fn left_paren(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &LEFT_PAREN_REGEX, TokenKind::LeftParen)
    }

    fn pipe(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &PIPE_REGEX, TokenKind::Pipe)
    }

    fn right_arrow(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &RIGHT_ARROW_REGEX, TokenKind::RightArrow)
    }

    fn right_brace(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &RIGHT_BRACE_REGEX, TokenKind::RightBrace)
    }

    fn right_bracket(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(
            input,
            position,
            &RIGHT_BRACKET_REGEX,
            TokenKind::RightBracket,
        )
    }

    fn right_paren(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &RIGHT_PAREN_REGEX, TokenKind::RightParen)
    }

    fn semicolon(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &SEMICOLON_REGEX, TokenKind::Semicolon)
    }

    fn underscore(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &UNDERSCORE_REGEX, TokenKind::Underscore)
    }

    // Comments are started with two slashes and end with a newline
    fn comment(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &COMMENT_REGEX, TokenKind::Comment)
    }

    fn doc_comment(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &DOC_COMMENT_REGEX, TokenKind::DocComment)
    }

    fn whitespace(input: &'a str, position: usize) -> Option<Token<'a>> {
        find_regex(input, position, &WHITESPACE_REGEX, TokenKind::WhiteSpace)
    }
}

fn find_regex<'a>(
    input: &'a str,
    position: usize,
    re: &Regex,
    kind: TokenKind,
) -> Option<Token<'a>> {
    re.find(input).map(|m| {
        let pos = Position::new(position + m.start(), position + m.end());

        println!("PROCESSING: {:?}", kind);

        Token::new(kind, m.as_str(), pos)
    })
}
