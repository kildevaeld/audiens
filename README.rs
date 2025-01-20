# Audiens

Generate visiting traits from enums


```rust

#[audiens::visitor(with_mut, with_fold)]
enum Expr {
    Call(CallExpr),
    Binary(BinaryExpr)
}

```


Will generate something like this:

```rust

pub trait ExprVisitor<'ast> {
    type Output;
    fn visit_call_expr(&mut self, member: &'ast CallExpr) -> Self::Output;
    fn visit_binary_expr(&mut self, member: &'ast BinaryExpr) -> Self::Output;
}

impl Expr {
    pub fn accept<'ast, V: ExprVisitor<'ast>>(&'ast self, visitor: &mut V) -> V::Output {
        match self {
            Self::Call(field_0) => visitor.visit_call_expr(field_0),
            Self::Binary(field_0) => visitor.visit_binary_expr(field_0),
        }
    }
}

pub trait ExprVisitorMut<'ast> {
    type Output;
    fn visit_mut_call_expr(&mut self, member: &'ast mut CallExpr) -> Self::Output;
    fn visit_mut_binary_expr(&mut self, member: &'ast mut BinaryExpr) -> Self::Output;
}

impl Expr {
    pub fn accept_mut<'ast, V: ExprVisitorMut<'ast>>(
        &'ast mut self,
        visitor: &mut V,
    ) -> V::Output {
        match self {
            Self::Call(field_0) => visitor.visit_mut_call_expr(field_0),
            Self::Binary(field_0) => visitor.visit_mut_binary_expr(field_0),
        }
    }
}

pub trait ExprFold {
    type Output;
    fn fold_call_expr(&mut self, member: CallExpr) -> Self::Output;
    fn fold_binary_expr(&mut self, member: BinaryExpr) -> Self::Output;
}

impl Expr {
    pub fn fold<V: ExprFold>(self, visitor: &mut V) -> V::Output {
        match self {
            Self::Call(field_0) => visitor.fold_call_expr(field_0),
            Self::Binary(field_0) => visitor.fold_binary_expr(field_0),
        }
    }
}

```