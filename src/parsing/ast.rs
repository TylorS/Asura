use crate::{lexing::Position, Type};

#[derive(Debug, PartialEq, Clone)]
pub enum AST<'a> {
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    NumberLiteral(NumberLiteral),
    RegexpLiteral(RegexpLiteral<'a>),
    StringLiteral(StringLiteral<'a>),
    TemplateLiteral(TemplateLiteral<'a>),
    Identifier(Identifier<'a>),
    ImportDeclaration(ImportDeclaration<'a>),
    Match(Match<'a>),
    FunctionDeclaration(FunctionDeclaration<'a>),
    TypeDeclaration(TypeDeclaration<'a>),
    TypeAlias(TypeAlias<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SourceFile<'a> {
    pub body: &'a Vec<AST<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLiteral {
    value: bool,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    value: i64,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberLiteral {
    value: f64,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegexpLiteral<'a> {
    value: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral<'a> {
    value: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TemplateLiteral<'a> {
    template: &'a Vec<&'a str>,
    values: &'a Vec<AST<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier<'a> {
    value: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration<'a> {
    name: &'a str, // TODO: Support named imports?
    specifier: &'a str,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Match<'a> {
    value: Identifier<'a>,
    position: Position,
    // TODO: Add cases
    // Predicates
    // Patterns
    // Catch-all
    // With and without blocks
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration<'a> {
    name: &'a str,
    type_parameters: Vec<TypeParameter<'a>>,
    parameters: Vec<FunctionParameter<'a>>,
    return_type: Option<TypeAnnotation<'a>>,
    body: Vec<AST<'a>>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda<'a> {
    type_parameters: Vec<TypeParameter<'a>>,
    parameters: Vec<FunctionParameter<'a>>,
    return_type: Option<TypeAnnotation<'a>>,
    body: Vec<AST<'a>>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter<'a> {
    name: &'a str,
    annotation: Option<TypeAnnotation<'a>>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeParameter<'a> {
    name: &'a str,
    extends: Option<Type<'a>>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDeclaration<'a> {
    name: &'a str,
    type_parameters: Vec<TypeParameter<'a>>,
    type_constructors: Vec<TypeConstructor<'a>>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeConstructor<'a> {
    name: &'a str,
    type_parameters: Vec<TypeParameter<'a>>,
    parameters: Vec<LabeledTypeAnnotation<'a>>,
    return_type: Option<TypeAnnotation<'a>>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAnnotation<'a> {
    annotated: Type<'a>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabeledTypeAnnotation<'a> {
    name: &'a str,
    annotated: TypeAnnotation<'a>,
    position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAlias<'a> {
    name: &'a str,
    type_parameters: Vec<TypeParameter<'a>>,
    aliased: TypeAnnotation<'a>,
    position: Position,
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
