use ast::{BinOp, UnOp};

#[derive(Debug, Clone)]
pub enum Bytecode {
    Int(i64),
    Float(f64),
    Bool(bool),
    Op2(BinOp),
    Op1(UnOp),
    Call(usize),
    Discard,
    Return(usize),
    PutLocal(usize),
    GetLocal(usize),
    Jump(isize),
    Branch(isize),
    //Alloc(),
    //Slice(),
}
