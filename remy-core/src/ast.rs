#[derive(Debug, PartialEq)]
pub struct File {
    pub(crate) definitions: Vec<TopLevelDefinition>,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelDefinition {
    Binding { lhs: BindingLeftHand, rhs: Literal },
    Extern { lhs: BindingLeftHand, rhs: Ident },
}

#[derive(Debug, PartialEq)]
pub struct BindingLeftHand {
    pub name: Ident,
    pub type_args: Vec<TypeName>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Function {
        args: Vec<AnnotatedIdent>,
        body: Vec<Expr>,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    FunctionCall(Box<Expr>, Vec<Expr>),
    Literal(Literal),
    Ident(Ident),
    BinaryOp {
        op: BinaryOperator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Match {
        target: Box<Expr>,
        conditions: Vec<(Literal, Expr)>,
    },
    Binding {
        ident: Ident,
        value: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Multiply,
    Divide,
    Subtract,
}

pub type Ident = String;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeName {
    Named(Ident),
    Slice(Box<TypeName>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedIdent {
    pub(crate) name: Ident,
    pub(crate) r#type: TypeName,
}
