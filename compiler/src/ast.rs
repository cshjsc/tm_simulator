
use std::collections::HashSet;

pub struct TmDef {
    identifier: String,
    alphabet: Vec<String>,
    block: TmBlock
}

pub struct TmBlock (Vec<TmStmt>);

impl TmBlock {
    pub fn new() -> TmBlock {
        TmBlock(Vec::new())
    }
}

pub enum TmStmt {
    Step(Vec<TmStep>),
    Branch { condition: TmStep, body: TmBlock },
    Cycle(TmBlock)
}

pub enum TmStep {
    ReplMove { 
        lhs: HashSet<String>,
        rhs: Option<String>,
        dir: TmDir,
    },
    Break,
    Halt
}

pub enum TmDir {
    Left,
    Right,
    Stay
}