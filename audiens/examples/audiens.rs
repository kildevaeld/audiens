use audiens::visitor;

pub struct CallExpr;

pub struct BinaryExpr;

#[audiens::visitor(with_mut, with_fold)]
enum Expr {
    Call(CallExpr),
    Binary(BinaryExpr),
}

fn main() {}
