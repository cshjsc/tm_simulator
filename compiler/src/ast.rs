use std::collections::HashMap;
use std::collections::HashSet;

use crate::tm::{Direction, Machine, State};

use either::Either;

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

#[derive(Debug)]
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TmOperation {
    Move {
        replace: Option<String>,
        direction: TmDir,
    },
    Break,
    Halt,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TmDir {
    Left,
    Right,
    Stay,
}

pub fn into_steps(def: TmDef) -> HashMap<u32, (TmStep, u32, u32)> {
    let mut state_counter = 2..;
    let mut queue = vec![(Either::Left(def.block), 1, 0, 0)];
    let mut steps = HashMap::new();

    while let Some((inst, curr, next, outer)) = queue.pop() {
        match inst {
            Either::Left(TmBlock(block)) => {
                let mut curr_st = curr;
                let mut instructions = block
                    .into_iter()
                    .map(|instruction| {
                        let next_st = state_counter.next().unwrap();
                        let res = (Either::Right(instruction), curr_st, next_st, outer);
                        curr_st = next_st;
                        res
                    })
                    .collect::<Vec<_>>();

                let len = instructions.len();
                instructions[len - 1].2 = next;
                queue.append(&mut instructions);
            }
            Either::Right(TmStmt::Cycle(block)) => {
                queue.push((Either::Left(block), curr, curr, next));
            }
            Either::Right(TmStmt::Branch { condition, body }) => {
                let body_st = state_counter.next().unwrap();
                queue.push((Either::Right(TmStmt::Step(condition)), curr, body_st, next));
                queue.push((Either::Left(body), body_st, outer, outer));
            }
            Either::Right(TmStmt::Step(step)) => {
                steps.insert(curr, (step, next, outer));
            }
        }
    }

    steps
}
