#[derive(Debug, PartialEq)]
pub struct TypedFile {
    pub(crate) definitions: Vec<TypedTopLevelDefinition>,
}

#[derive(Debug, PartialEq)]
pub enum TypedTopLevelDefinition {
    Binding {
        lhs: TypedBindingLeftHand,
        rhs: TypedLiteral,
    },
    Extern {
        name: Ident,
        rhs: Type,
    },
}

#[derive(Debug, PartialEq)]
pub struct BindingLeftHand {
    pub name: Ident,
    pub type_args: Vec<ConstrainedType>,
}
#[derive(Debug, PartialEq)]
pub struct ConstrainedType {
    pub name: Ident,
    pub constraints: Vec<TypeConstraint>,
}
pub type TypeConstraint = Ident;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Function {
        args: Vec<AnnotatedIdent>,
        ret_type: TypeName,
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
pub enum Type {
    Function { args: Vec<Type>, ret: Box<Type> },
    Struct { fields: HashMap<String, Type> },
    Slice(Box<Type>),
    Array { contained: Box<Type>, len: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedIdent {
    pub(crate) name: Ident,
    pub(crate) r#type: TypeName,
}
