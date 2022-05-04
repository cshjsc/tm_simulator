use std::collections::HashMap;
use std::collections::HashSet;

pub struct TmDef {
    identifier: String,
    alphabet: HashSet<String>,
    block: TmBlock,
}

impl TmDef {
    pub fn new(identifier: String, alphabet: HashSet<String>, block: TmBlock) -> TmDef {
        TmDef {
            identifier,
            alphabet,
            block,
        }
    }
}

pub struct TmBlock(Vec<TmStmt>);

impl TmBlock {
    pub fn new(stats: Vec<TmStmt>) -> TmBlock {
        TmBlock(stats)
    }
}

pub enum TmStmt {
    Step(TmStep),
    Branch { condition: TmStep, body: TmBlock },
    Cycle(TmBlock),
}

pub struct AtomicTmStep {
    patterns: HashSet<String>,
    operation: TmOperation,
}

impl AtomicTmStep {
    pub fn new(patterns: Vec<String>, repl: Option<String>, dir: TmDir) -> AtomicTmStep {
        AtomicTmStep {
            patterns: patterns.into_iter().collect(),
            operation: TmOperation::Move {
                replace: repl,
                direction: dir,
            },
        }
    }
}

pub struct TmStep {
    cases: HashMap<String, TmOperation>,
    default: Option<TmOperation>,
}

impl TmStep {
    pub fn new(atomic_steps: Vec<AtomicTmStep>, default: Option<TmOperation>) -> TmStep {
        let mut cases = HashMap::new();

        for atomic_step in atomic_steps {
            for pattern in atomic_step.patterns {
                cases.insert(pattern, atomic_step.operation.clone());
            }
        }

        TmStep { cases, default }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum TmOperation {
    Move {
        replace: Option<String>,
        direction: TmDir,
    },
    Break,
    Halt,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TmDir {
    Left,
    Right,
    Stay,
}
