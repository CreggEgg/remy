#[derive(Debug, PartialEq)]
pub struct File {
    pub(crate) definitions: Vec<TopLevelDefinition>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelDefinition {
    Binding { name: Ident, rhs: Literal },
}
#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Function {
        args: Vec<AnnotatedIdent>,
        body: Vec<Expr>,
    },
}
#[derive(Debug, PartialEq)]
pub enum Expr {
    FunctionCall(Box<Expr>, Vec<Expr>),
    Literal(Literal),
    Ident(Ident),
}

pub type Ident = String;

#[derive(Debug, PartialEq)]
pub enum TypeName {
    Named(Ident),
    Slice(Box<TypeName>),
}

#[derive(Debug, PartialEq)]
pub struct AnnotatedIdent {
    pub(crate) name: Ident,
    pub(crate) r#type: TypeName,
}
