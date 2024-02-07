# audiens


Create visitors for enums:


```rust

#[visitor]
enum Expr {
    Binary(BinaryExpr),
    Value(ValueExpr)
}

```

Generates:


```rust

pub trait ExprVisitor {
    type Output
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Output;
    fn visit_value_expr(&mut self, expr: &ValueExpr) -> Self::Output;
}


impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Self::Binary(e) => visitor.visit_binary_expr(e),
            ...
        }
    }
}

```