// TODO: Refinment types?
// TODO: Mapped types?

#[derive(Debug, PartialEq, Clone)]
pub enum Type<'a> {
    Unit,    // Nothing
    Never,   // Bottom
    Unknown, // Top
    Boolean,
    Int,
    Float,
    String,
    Regexp(&'a str),
    Template(Vec<Type<'a>>),
    Brand(&'a str), // Must have a label
    Option(Box<Type<'a>>),
    Either(Box<Type<'a>>, Box<Type<'a>>),
    Array(Box<Type<'a>>),
    Tuple(Vec<TupleData<'a>>),
    Struct(Vec<StructData<'a>>),
    Function(
        /*input type params*/ Vec<Type<'a>>,
        /*Argument types*/ Vec<Type<'a>>,
        /*Effect Types*/ Vec<Type<'a>>,
        /*Return Types*/ Box<Type<'a>>,
    ),
    Alias(&'a str, Box<Type<'a>>), // To preserve the name of the type in hovers
    Union(Vec<Type<'a>>),
    Intersection(Vec<Type<'a>>),
    Infer,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TupleData<'a> {
    Member(Box<Type<'a>>),
    Spread(Box<TupleData<'a>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructData<'a> {
    Member(&'a str, Box<Type<'a>>),
    Spread(Box<StructData<'a>>),
}
