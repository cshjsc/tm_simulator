
use std::collections::HashMap;
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

pub struct AtomicTmStep {
    patterns: HashSet<String>,
    operation: TmOperation
}

impl AtomicTmStep {
    pub fn new(
        patterns: Vec<String>, 
        repl: Option<String>, 
        dir: TmDir) 
        -> AtomicTmStep 
    {
        AtomicTmStep {
            patterns: patterns.into_iter().collect(),
            operation: TmOperation::Move {
                replace: repl,
                direction: dir
            }
        }
    }
}

pub struct TmStep{
    map: HashMap<String, TmOperation>,
    default: Option<TmOperation>
}

impl FromIterator<AtomicTmStep> for TmStep {
    fn from_iter<T: IntoIterator<Item = AtomicTmStep>>(_: T) -> Self { 
        todo!() 
    }
}

pub enum TmOperation {
    Move { 
        replace: Option<String>, 
        direction: TmDir
    },
    Break,
    Halt
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TmDir {
    Left,
    Right,
    Stay
}