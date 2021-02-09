use std::rc::Rc;

use super::*;
use crate::color::RgbaColor;
use crate::geom::{AngularUnit, LengthUnit};

/// An expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A literal.
    Lit(Lit),
    /// An identifier: `left`.
    Ident(Ident),
    /// An array expression: `(1, "hi", 12cm)`.
    Array(ExprArray),
    /// A dictionary expression: `(color: #f79143, pattern: dashed)`.
    Dict(ExprDict),
    /// A template expression: `[*Hi* there!]`.
    Template(ExprTemplate),
    /// A grouped expression: `(1 + 2)`.
    Group(ExprGroup),
    /// A block expression: `{ #let x = 1; x + 2 }`.
    Block(ExprBlock),
    /// A unary operation: `-x`.
    Unary(ExprUnary),
    /// A binary operation: `a + b`.
    Binary(ExprBinary),
    /// An invocation of a function: `foo(...)`, `#[foo ...]`.
    Call(ExprCall),
    /// A let expression: `#let x = 1`.
    Let(ExprLet),
    /// An if expression: `#if x { y } #else { z }`.
    If(ExprIf),
    /// A for expression: `#for x #in y { z }`.
    For(ExprFor),
}

impl Expr {
    /// The source code location.
    pub fn span(&self) -> Span {
        match self {
            Self::Lit(v) => v.span,
            Self::Ident(v) => v.span,
            Self::Array(v) => v.span,
            Self::Dict(v) => v.span,
            Self::Template(v) => v.span,
            Self::Group(v) => v.span,
            Self::Block(v) => v.span,
            Self::Unary(v) => v.span,
            Self::Binary(v) => v.span,
            Self::Call(v) => v.span,
            Self::Let(v) => v.span,
            Self::If(v) => v.span,
            Self::For(v) => v.span,
        }
    }
}

impl Pretty for Expr {
    fn pretty(&self, p: &mut Printer) {
        match self {
            Self::Lit(v) => v.pretty(p),
            Self::Ident(v) => v.pretty(p),
            Self::Array(v) => v.pretty(p),
            Self::Dict(v) => v.pretty(p),
            Self::Template(v) => v.pretty(p),
            Self::Group(v) => v.pretty(p),
            Self::Block(v) => v.pretty(p),
            Self::Unary(v) => v.pretty(p),
            Self::Binary(v) => v.pretty(p),
            Self::Call(v) => v.pretty(p),
            Self::Let(v) => v.pretty(p),
            Self::If(v) => v.pretty(p),
            Self::For(v) => v.pretty(p),
        }
    }
}

/// A literal.
#[derive(Debug, Clone, PartialEq)]
pub struct Lit {
    /// The source code location.
    pub span: Span,
    /// The kind of literal.
    pub kind: LitKind,
}

impl Pretty for Lit {
    fn pretty(&self, p: &mut Printer) {
        self.kind.pretty(p);
    }
}

/// A kind of literal.
#[derive(Debug, Clone, PartialEq)]
pub enum LitKind {
    /// The none literal: `none`.
    None,
    /// A boolean literal: `true`, `false`.
    Bool(bool),
    /// An integer literal: `120`.
    Int(i64),
    /// A floating-point literal: `1.2`, `10e-4`.
    Float(f64),
    /// A length literal: `12pt`, `3cm`.
    Length(f64, LengthUnit),
    /// An angle literal:  `1.5rad`, `90deg`.
    Angle(f64, AngularUnit),
    /// A percent literal: `50%`.
    ///
    /// _Note_: `50%` is stored as `50.0` here, but as `0.5` in the
    /// corresponding [value](crate::geom::Relative).
    Percent(f64),
    /// A color literal: `#ffccee`.
    Color(RgbaColor),
    /// A string literal: `"hello!"`.
    Str(String),
}

impl Pretty for LitKind {
    fn pretty(&self, p: &mut Printer) {
        match self {
            Self::None => p.push_str("none"),
            Self::Bool(v) => v.pretty(p),
            Self::Int(v) => v.pretty(p),
            Self::Float(v) => v.pretty(p),
            Self::Length(v, u) => {
                write!(p, "{}{}", ryu::Buffer::new().format(*v), u).unwrap();
            }
            Self::Angle(v, u) => {
                write!(p, "{}{}", ryu::Buffer::new().format(*v), u).unwrap();
            }
            Self::Percent(v) => {
                write!(p, "{}%", ryu::Buffer::new().format(*v)).unwrap();
            }
            Self::Color(v) => v.pretty(p),
            Self::Str(v) => v.pretty(p),
        }
    }
}

/// An array expression: `(1, "hi", 12cm)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprArray {
    /// The source code location.
    pub span: Span,
    /// The entries of the array.
    pub items: Vec<Expr>,
}

impl Pretty for ExprArray {
    fn pretty(&self, p: &mut Printer) {
        p.push('(');
        p.join(&self.items, ", ", |item, p| item.pretty(p));
        if self.items.len() == 1 {
            p.push(',');
        }
        p.push(')');
    }
}

/// A dictionary expression: `(color: #f79143, pattern: dashed)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprDict {
    /// The source code location.
    pub span: Span,
    /// The named dictionary entries.
    pub items: Vec<Named>,
}

impl Pretty for ExprDict {
    fn pretty(&self, p: &mut Printer) {
        p.push('(');
        if self.items.is_empty() {
            p.push(':');
        } else {
            p.join(&self.items, ", ", |named, p| named.pretty(p));
        }
        p.push(')');
    }
}

/// A pair of a name and an expression: `pattern: dashed`.
#[derive(Debug, Clone, PartialEq)]
pub struct Named {
    /// The name: `pattern`.
    pub name: Ident,
    /// The right-hand side of the pair: `dashed`.
    pub expr: Expr,
}

impl Named {
    /// The source code location.
    pub fn span(&self) -> Span {
        self.name.span.join(self.expr.span())
    }
}

impl Pretty for Named {
    fn pretty(&self, p: &mut Printer) {
        self.name.pretty(p);
        p.push_str(": ");
        self.expr.pretty(p);
    }
}

/// A template expression: `[*Hi* there!]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprTemplate {
    /// The source code location.
    pub span: Span,
    /// The contents of the template.
    pub tree: Rc<Tree>,
}

impl Pretty for ExprTemplate {
    fn pretty(&self, p: &mut Printer) {
        if let [Node::Expr(Expr::Call(call))] = self.tree.as_slice() {
            call.pretty_bracketed(p, false);
        } else {
            p.push('[');
            self.tree.pretty(p);
            p.push(']');
        }
    }
}

/// A grouped expression: `(1 + 2)`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprGroup {
    /// The source code location.
    pub span: Span,
    /// The wrapped expression.
    pub expr: Box<Expr>,
}

impl Pretty for ExprGroup {
    fn pretty(&self, p: &mut Printer) {
        p.push('(');
        self.expr.pretty(p);
        p.push(')');
    }
}

/// A block expression: `{ #let x = 1; x + 2 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBlock {
    /// The source code location.
    pub span: Span,
    /// The list of expressions contained in the block.
    pub exprs: Vec<Expr>,
    /// Whether the block should create a scope.
    pub scoping: bool,
}

impl Pretty for ExprBlock {
    fn pretty(&self, p: &mut Printer) {
        p.push('{');
        if self.exprs.len() > 1 {
            p.push(' ');
        }
        p.join(&self.exprs, "; ", |expr, p| expr.pretty(p));
        if self.exprs.len() > 1 {
            p.push(' ');
        }
        p.push('}');
    }
}

/// A unary operation: `-x`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprUnary {
    /// The source code location.
    pub span: Span,
    /// The operator: `-`.
    pub op: UnOp,
    /// The expression to operator on: `x`.
    pub expr: Box<Expr>,
}

impl Pretty for ExprUnary {
    fn pretty(&self, p: &mut Printer) {
        self.op.pretty(p);
        if self.op == UnOp::Not {
            p.push(' ');
        }
        self.expr.pretty(p);
    }
}

/// A unary operator.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnOp {
    /// The plus operator: `+`.
    Pos,
    /// The negation operator: `-`.
    Neg,
    /// The boolean `not`.
    Not,
}

impl UnOp {
    /// Try to convert the token into a unary operation.
    pub fn from_token(token: Token) -> Option<Self> {
        Some(match token {
            Token::Plus => Self::Pos,
            Token::Hyph => Self::Neg,
            Token::Not => Self::Not,
            _ => return None,
        })
    }

    /// The precedence of this operator.
    pub fn precedence(self) -> usize {
        match self {
            Self::Pos | Self::Neg => 8,
            Self::Not => 4,
        }
    }

    /// The string representation of this operation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pos => "+",
            Self::Neg => "-",
            Self::Not => "not",
        }
    }
}

impl Pretty for UnOp {
    fn pretty(&self, p: &mut Printer) {
        p.push_str(self.as_str());
    }
}

/// A binary operation: `a + b`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    /// The source code location.
    pub span: Span,
    /// The left-hand side of the operation: `a`.
    pub lhs: Box<Expr>,
    /// The operator: `+`.
    pub op: BinOp,
    /// The right-hand side of the operation: `b`.
    pub rhs: Box<Expr>,
}

impl Pretty for ExprBinary {
    fn pretty(&self, p: &mut Printer) {
        self.lhs.pretty(p);
        p.push(' ');
        self.op.pretty(p);
        p.push(' ');
        self.rhs.pretty(p);
    }
}

/// A binary operator.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinOp {
    /// The addition operator: `+`.
    Add,
    /// The subtraction operator: `-`.
    Sub,
    /// The multiplication operator: `*`.
    Mul,
    /// The division operator: `/`.
    Div,
    /// The short-circuiting boolean `and`.
    And,
    /// The short-circuiting boolean `or`.
    Or,
    /// The equality operator: `==`.
    Eq,
    /// The inequality operator: `!=`.
    Neq,
    /// The less-than operator: `<`.
    Lt,
    /// The less-than or equal operator: `<=`.
    Leq,
    /// The greater-than operator: `>`.
    Gt,
    /// The greater-than or equal operator: `>=`.
    Geq,
    /// The assignment operator: `=`.
    Assign,
    /// The add-assign operator: `+=`.
    AddAssign,
    /// The subtract-assign oeprator: `-=`.
    SubAssign,
    /// The multiply-assign operator: `*=`.
    MulAssign,
    /// The divide-assign operator: `/=`.
    DivAssign,
}

impl BinOp {
    /// Try to convert the token into a binary operation.
    pub fn from_token(token: Token) -> Option<Self> {
        Some(match token {
            Token::Plus => Self::Add,
            Token::Hyph => Self::Sub,
            Token::Star => Self::Mul,
            Token::Slash => Self::Div,
            Token::And => Self::And,
            Token::Or => Self::Or,
            Token::EqEq => Self::Eq,
            Token::BangEq => Self::Neq,
            Token::Lt => Self::Lt,
            Token::LtEq => Self::Leq,
            Token::Gt => Self::Gt,
            Token::GtEq => Self::Geq,
            Token::Eq => Self::Assign,
            Token::PlusEq => Self::AddAssign,
            Token::HyphEq => Self::SubAssign,
            Token::StarEq => Self::MulAssign,
            Token::SlashEq => Self::DivAssign,
            _ => return None,
        })
    }

    /// The precedence of this operator.
    pub fn precedence(self) -> usize {
        match self {
            Self::Mul | Self::Div => 7,
            Self::Add | Self::Sub => 6,
            Self::Eq | Self::Neq | Self::Lt | Self::Leq | Self::Gt | Self::Geq => 5,
            Self::And => 3,
            Self::Or => 2,
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign => 1,
        }
    }

    /// The associativity of this operator.
    pub fn associativity(self) -> Associativity {
        match self {
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::And
            | Self::Or
            | Self::Eq
            | Self::Neq
            | Self::Lt
            | Self::Leq
            | Self::Gt
            | Self::Geq => Associativity::Left,
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign => Associativity::Right,
        }
    }

    /// The string representation of this operation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::And => "and",
            Self::Or => "or",
            Self::Eq => "==",
            Self::Neq => "!=",
            Self::Lt => "<",
            Self::Leq => "<=",
            Self::Gt => ">",
            Self::Geq => ">=",
            Self::Assign => "=",
            Self::AddAssign => "+=",
            Self::SubAssign => "-=",
            Self::MulAssign => "*=",
            Self::DivAssign => "/=",
        }
    }
}

impl Pretty for BinOp {
    fn pretty(&self, p: &mut Printer) {
        p.push_str(self.as_str());
    }
}

/// The associativity of a binary operator.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Associativity {
    /// Left-associative: `a + b + c` is equivalent to `(a + b) + c`.
    Left,
    /// Right-associative: `a = b = c` is equivalent to `a = (b = c)`.
    Right,
}

/// An invocation of a function: `foo(...)`, `#[foo ...]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    /// The source code location.
    pub span: Span,
    /// The callee of the function.
    pub callee: Box<Expr>,
    /// The arguments to the function.
    pub args: ExprArgs,
}

impl Pretty for ExprCall {
    fn pretty(&self, p: &mut Printer) {
        self.callee.pretty(p);
        p.push('(');
        self.args.pretty(p);
        p.push(')');
    }
}

impl ExprCall {
    /// Pretty print a function template, with body or chaining when possible.
    pub fn pretty_bracketed(&self, p: &mut Printer, chained: bool) {
        if chained {
            p.push_str(" | ");
        } else {
            p.push_str("#[");
        }

        // Function name.
        self.callee.pretty(p);

        let mut write_args = |items: &[Argument]| {
            if !items.is_empty() {
                p.push(' ');
                p.join(items, ", ", |item, p| item.pretty(p));
            }
        };

        match self.args.items.as_slice() {
            // This can written as a chain.
            //
            // Example: Transforms "#[v][[f]]" => "#[v | f]".
            [head @ .., Argument::Pos(Expr::Call(call))] => {
                write_args(head);
                call.pretty_bracketed(p, true);
            }

            // This can be written with a body.
            //
            // Example: Transforms "#[v [Hi]]" => "#[v][Hi]".
            [head @ .., Argument::Pos(Expr::Template(template))] => {
                write_args(head);
                p.push(']');
                template.pretty(p);
            }

            items => {
                write_args(items);
                p.push(']');
            }
        }
    }
}

/// The arguments to a function: `12, draw: false`.
///
/// In case of a bracketed invocation with a body, the body is _not_
/// included in the span for the sake of clearer error messages.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprArgs {
    /// The source code location.
    pub span: Span,
    /// The positional and named arguments.
    pub items: Vec<Argument>,
}

impl Pretty for ExprArgs {
    fn pretty(&self, p: &mut Printer) {
        p.join(&self.items, ", ", |item, p| item.pretty(p));
    }
}

/// An argument to a function call: `12` or `draw: false`.
#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    /// A positional arguments.
    Pos(Expr),
    /// A named argument.
    Named(Named),
}

impl Argument {
    /// The source code location.
    pub fn span(&self) -> Span {
        match self {
            Self::Pos(expr) => expr.span(),
            Self::Named(named) => named.span(),
        }
    }
}

impl Pretty for Argument {
    fn pretty(&self, p: &mut Printer) {
        match self {
            Self::Pos(expr) => expr.pretty(p),
            Self::Named(named) => named.pretty(p),
        }
    }
}

/// A let expression: `#let x = 1`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprLet {
    /// The source code location.
    pub span: Span,
    /// The binding to assign to.
    pub binding: Ident,
    /// The expression the pattern is initialized with.
    pub init: Option<Box<Expr>>,
}

impl Pretty for ExprLet {
    fn pretty(&self, p: &mut Printer) {
        p.push_str("#let ");
        self.binding.pretty(p);
        if let Some(init) = &self.init {
            p.push_str(" = ");
            init.pretty(p);
        }
    }
}

/// An if expression: `#if x { y } #else { z }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprIf {
    /// The source code location.
    pub span: Span,
    /// The condition which selects the body to evaluate.
    pub condition: Box<Expr>,
    /// The expression to evaluate if the condition is true.
    pub if_body: Box<Expr>,
    /// The expression to evaluate if the condition is false.
    pub else_body: Option<Box<Expr>>,
}

impl Pretty for ExprIf {
    fn pretty(&self, p: &mut Printer) {
        p.push_str("#if ");
        self.condition.pretty(p);
        p.push(' ');
        self.if_body.pretty(p);
        if let Some(expr) = &self.else_body {
            p.push_str(" #else ");
            expr.pretty(p);
        }
    }
}

/// A for expression: `#for x #in y { z }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExprFor {
    /// The source code location.
    pub span: Span,
    /// The pattern to assign to.
    pub pattern: ForPattern,
    /// The expression to iterate over.
    pub iter: Box<Expr>,
    /// The expression to evaluate for each iteration.
    pub body: Box<Expr>,
}

impl Pretty for ExprFor {
    fn pretty(&self, p: &mut Printer) {
        p.push_str("#for ");
        self.pattern.pretty(p);
        p.push_str(" #in ");
        self.iter.pretty(p);
        p.push(' ');
        self.body.pretty(p);
    }
}

/// A pattern in a for loop.
#[derive(Debug, Clone, PartialEq)]
pub enum ForPattern {
    /// A value pattern: `#for v #in array`.
    Value(Ident),
    /// A key-value pattern: `#for k, v #in dict`.
    KeyValue(Ident, Ident),
}

impl ForPattern {
    /// The source code location.
    pub fn span(&self) -> Span {
        match self {
            Self::Value(v) => v.span,
            Self::KeyValue(k, v) => k.span.join(v.span),
        }
    }
}

impl Pretty for ForPattern {
    fn pretty(&self, p: &mut Printer) {
        match self {
            Self::Value(v) => v.pretty(p),
            Self::KeyValue(k, v) => {
                k.pretty(p);
                p.push_str(", ");
                v.pretty(p);
            }
        }
    }
}
