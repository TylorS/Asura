#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    BooleanLiteral,
    NumberLiteral,
    RegexpLiteral,
    StringLiteral,
    TemplateLiteral,

    // Identifiers
    Identifier,
    MacroIdentifier,

    // Keywords
    Alias,
    Brand,
    Case,
    Effect,
    Export,
    Else,
    ElseIf,
    For,
    From,
    Function,
    If,
    Import,
    Macro,
    Match,
    Return,
    Struct,
    Type,
    Typeclass,
    While,
    With,

    // Symbols + Operators
    And,
    AndAnd,
    Backslash,
    Bang,
    BangEqual,
    Caret,
    Dollar,
    Equal,
    EqualEqual,
    GreaterThan,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
    GreaterThanEqual,
    Hash,
    LessThan,
    LessThanLessThan,
    LessThanEqual,
    Minus,
    MinusMinus,
    Or,
    OrOr,
    Percent,
    Plus,
    PlusPlus,
    Question,
    Slash,
    Star,
    StarStar,
    Tilde,

    // Delimiters
    At,
    Colon,
    Comma,
    Dot,
    DotDot,
    DotDotDot,
    FatArrow,
    LeftArrow,
    LeftBrace,
    LeftBracket,
    LeftParen,
    Pipe,
    RightBrace,
    RightBracket,
    RightParen,
    Semicolon,
    Underscore,

    // Whitespace
    WhiteSpace,

    // Comments
    Comment,
    DocComment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    kind: TokenKind,
    value: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, value: &'a str, position: Position) -> Token<'a> {
        Token {
            kind,
            value,
            position,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn value(&self) -> &'a str {
        self.value
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn boolean_literal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::BooleanLiteral, value, position)
    }

    pub fn number_literal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::NumberLiteral, value, position)
    }

    pub fn regexp_literal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::RegexpLiteral, value, position)
    }

    pub fn string_literal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::StringLiteral, value, position)
    }

    pub fn template_literal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::TemplateLiteral, value, position)
    }

    pub fn identifier(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Identifier, value, position)
    }

    pub fn macro_identifier(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::MacroIdentifier, value, position)
    }

    pub fn alias(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Alias, value, position)
    }

    pub fn brand(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Brand, value, position)
    }

    pub fn case(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Case, value, position)
    }

    pub fn effect(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Effect, value, position)
    }

    pub fn export(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Export, value, position)
    }

    pub fn else_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Else, value, position)
    }

    pub fn else_if(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::ElseIf, value, position)
    }

    pub fn for_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::For, value, position)
    }

    pub fn from(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::From, value, position)
    }

    pub fn function(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Function, value, position)
    }

    pub fn if_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::If, value, position)
    }

    pub fn import(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Import, value, position)
    }

    pub fn macro_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Macro, value, position)
    }

    pub fn match_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Match, value, position)
    }

    pub fn return_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Return, value, position)
    }

    pub fn struct_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Struct, value, position)
    }

    pub fn type_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Type, value, position)
    }

    pub fn typeclass(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Typeclass, value, position)
    }

    pub fn while_(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::While, value, position)
    }

    pub fn with(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::With, value, position)
    }

    pub fn and(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::And, value, position)
    }

    pub fn and_and(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::AndAnd, value, position)
    }

    pub fn back_slash(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Backslash, value, position)
    }

    pub fn bang(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Bang, value, position)
    }

    pub fn bang_equal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::BangEqual, value, position)
    }

    pub fn caret(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Caret, value, position)
    }

    pub fn dollar(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Dollar, value, position)
    }

    pub fn equal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Equal, value, position)
    }

    pub fn equal_equal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::EqualEqual, value, position)
    }

    pub fn greater_than(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::GreaterThan, value, position)
    }

    pub fn greater_than_greater_than(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::GreaterThanGreaterThan, value, position)
    }

    pub fn greater_than_greater_than_greater_than(value: &'a str, position: Position) -> Token<'a> {
        Token::new(
            TokenKind::GreaterThanGreaterThanGreaterThan,
            value,
            position,
        )
    }

    pub fn greater_than_equal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::GreaterThanEqual, value, position)
    }

    pub fn hash(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Hash, value, position)
    }

    pub fn less_than(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::LessThan, value, position)
    }

    pub fn less_than_less_than(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::LessThanLessThan, value, position)
    }

    pub fn less_than_equal(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::LessThanEqual, value, position)
    }

    pub fn minus(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Minus, value, position)
    }

    pub fn minus_minus(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::MinusMinus, value, position)
    }

    pub fn or(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Or, value, position)
    }

    pub fn or_or(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::OrOr, value, position)
    }

    pub fn percent(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Percent, value, position)
    }

    pub fn plus(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Plus, value, position)
    }

    pub fn plus_plus(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::PlusPlus, value, position)
    }

    pub fn question(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Question, value, position)
    }

    pub fn slash(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Slash, value, position)
    }

    pub fn star(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Star, value, position)
    }

    pub fn star_star(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::StarStar, value, position)
    }

    pub fn tilde(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Tilde, value, position)
    }

    pub fn at(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::At, value, position)
    }

    pub fn colon(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Colon, value, position)
    }

    pub fn comma(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Comma, value, position)
    }

    pub fn dot(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Dot, value, position)
    }

    pub fn dot_dot(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::DotDot, value, position)
    }

    pub fn dot_dot_dot(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::DotDotDot, value, position)
    }

    pub fn fat_arrow(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::FatArrow, value, position)
    }

    pub fn left_arrow(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::LeftArrow, value, position)
    }

    pub fn left_brace(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::LeftBrace, value, position)
    }

    pub fn left_bracket(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::LeftBracket, value, position)
    }

    pub fn left_paren(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::LeftParen, value, position)
    }

    pub fn pipe(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Pipe, value, position)
    }

    pub fn right_brace(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::RightBrace, value, position)
    }

    pub fn right_bracket(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::RightBracket, value, position)
    }

    pub fn right_paren(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::RightParen, value, position)
    }

    pub fn semicolon(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Semicolon, value, position)
    }

    pub fn underscore(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Underscore, value, position)
    }

    pub fn whitespace(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::WhiteSpace, value, position)
    }

    pub fn comment(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::Comment, value, position)
    }

    pub fn doc_comment(value: &'a str, position: Position) -> Token<'a> {
        Token::new(TokenKind::DocComment, value, position)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(start: usize, end: usize) -> Position {
        Position { start, end }
    }
}
