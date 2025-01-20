use audiens::visitor;

#[derive(Debug)]
#[visitor(with_fold, with_mut)]
pub enum Enum<'a, T> {
    First(String),
    Next(u32, i32),
    Last { name: String, age: u16 },
    Name(&'a str),
    Var { target: T },
}

fn main() {}
