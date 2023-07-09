#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    BooleanLiteral(bool, Position),
    NumberLiteral(String, Position),
    RegexpLiteral(String, Position),
    StringLiteral(String, Position),
    TemplateLiteral(String, Position),

    // Identifiers
    Identifier(String, Position),
    MacroIdentifier(String, Position),

    // Keywords
    Alias(Position),
    Brand(Position),
    Case(Position),
    Effect(Position),
    Export(Position),
    Else(Position),
    ElseIf(Position),
    For(Position),
    From(Position),
    Function(Position),
    If(Position),
    Import(Position),
    Macro(Position),
    Match(Position),
    Return(Position),
    Struct(Position),
    Type(Position),
    Typeclass(Position),
    While(Position),
    With(Position),

    // Symbols + Operators
    And(Position),
    AndAnd(Position),
    Backslash(Position),
    Bang(Position),
    BangEqual(Position),
    Caret(Position),
    Dollar(Position),
    Equal(Position),
    EqualEqual(Position),
    GreaterThan(Position),
    GreaterThanGreaterThan(Position),
    GreaterThanGreaterThanGreaterThan(Position),
    GreaterThanEqual(Position),
    Hash(Position),
    LessThan(Position),
    LessThanLessThan(Position),
    LessThanEqual(Position),
    Minus(Position),
    Or(Position),
    OrOr(Position),
    Percent(Position),
    Plus(Position),
    PlusPlus(Position),
    Question(Position),
    Slash(Position),
    Star(Position),
    StarStar(Position),
    Tilde(Position),

    // Delimiters
    At(Position),
    Colon(Position),
    Comma(Position),
    CurlyLeftBrace(Position),
    CurlyRightBrace(Position),
    Dot(Position),
    LeftBrace(Position),
    LeftParen(Position),
    RightBrace(Position),
    RightParen(Position),
    Semicolon(Position),
    WhiteSpace(String, Position),

    // Synthetic
    Newline(String, Position),
    Eof(Position),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}
