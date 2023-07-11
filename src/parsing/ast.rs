use crate::lexing::Position;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AST<'a> {
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    NumberLiteral(NumberLiteral),
    RegexpLiteral(RegexpLiteral<'a>),
    StringLiteral(StringLiteral<'a>),
    TemplateLiteral(TemplateLiteral<'a>),
    Identifier(Identifier<'a>),
    ImportDeclaration(ImportDeclaration<'a>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SourceFile<'a> {
    pub body: &'a Vec<AST<'a>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BooleanLiteral {
    value: bool,
    position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IntegerLiteral {
    value: i64,
    position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NumberLiteral {
    value: f64,
    position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RegexpLiteral<'a> {
    value: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StringLiteral<'a> {
    value: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TemplateLiteral<'a> {
    template: &'a Vec<&'a str>,
    values: &'a Vec<AST<'a>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Identifier<'a> {
    value: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ImportDeclaration<'a> {
    name: &'a str, // TODO: Support named imports?
    specifier: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Match<'a> {
    value: Identifier<'a>,
    position: Position,
    // TODO: Add cases
    // Predicates
    // Patterns
    // Catch-all
    // With and without blocks
}

impl<'a> AST<'a> {
    pub fn boolean_literal(value: bool, position: Position) -> Self {
        AST::BooleanLiteral(BooleanLiteral { value, position })
    }

    pub fn integer_literal(value: i64, position: Position) -> Self {
        AST::IntegerLiteral(IntegerLiteral { value, position })
    }

    pub fn number_literal(value: f64, position: Position) -> Self {
        AST::NumberLiteral(NumberLiteral { value, position })
    }

    pub fn regexp_literal(value: &'a str, position: Position) -> Self {
        AST::RegexpLiteral(RegexpLiteral { value, position })
    }

    pub fn string_literal(value: &'a str, position: Position) -> Self {
        AST::StringLiteral(StringLiteral { value, position })
    }

    pub fn template_literal(template: &'a Vec<&'a str>, values: &'a Vec<AST<'a>>) -> Self {
        AST::TemplateLiteral(TemplateLiteral { template, values })
    }
}
