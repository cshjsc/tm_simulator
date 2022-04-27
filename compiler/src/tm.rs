use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Right,
    Left,
    Unchanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionFunction {
    pub origin: String,
    pub bands_requirements: Vec<String>,
    pub bands_actions: Vec<(String, Direction)>,
    pub next_state_name: String,
}

impl TransitionFunction {
    pub fn new(
        origin: String,
        bands_requirements: Vec<String>,
        bands_actions: Vec<(String, Direction)>,
        next_state_name: String,
    ) -> Self {
        if bands_requirements.len() != bands_actions.len() {
            //TODO message
            panic!("invalid bands");
        }
        Self {
            origin,
            bands_requirements,
            bands_actions,
            next_state_name,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub transition_functions: Vec<TransitionFunction>,
    pub is_end_state: bool,
}
impl State {
    pub fn new(transition_functions: Vec<TransitionFunction>, is_end_state: bool) -> Self {
        Self {
            transition_functions,
            is_end_state,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Machine {
    pub alphabet: HashSet<String>,
    pub size: usize,
    pub states: HashMap<String, State>,
    pub start_state_name: String,
}
impl Machine {
    pub fn new(
        alphabet: HashSet<String>,
        size: usize,
        states: HashMap<String, State>,
        start_state: String,
        //end_state: &'a str,
        //transition_functions: Vec<&'a TransitionFunction<'a>>,
    ) -> Result<Self, &'static str> {
        //TODO make this more idiomatic? and refactor
        //TODO make sure the machine has a way to end
        let mut alphabet = alphabet;
        alphabet.insert("_".to_string());
        if !states.contains_key(&start_state) {
            return Err("origin not in states");
        }
        for (_name, state) in states.iter() {
            for f in state.transition_functions.iter() {
                if f.bands_requirements.len() != size {
                    return Err(
                    "number of input bands for this function doesn't match the machine band size ",
                );
                }
                for requirement in f.bands_requirements.iter() {
                    if !alphabet.contains(requirement) {
                        return Err("character not defined in alphabet");
                    }
                }
                if f.bands_actions.len() != size {
                    return Err(
                        "number of bands for this function doesn't match the machines band size ",
                    );
                }
                if !states.contains_key(&f.next_state_name) {
                    return Err("next state not found");
                }
                for action in f.bands_actions.iter() {
                    if !alphabet.contains(&action.0) {
                        return Err("character not defined in alphabet");
                    }
                }
            }
        }

        Ok(Self {
            alphabet,
            size,
            states,
            start_state_name: start_state,
        })
    }
}

/**
 * Machine definition format
 * each statement ends with a semicolon followed by an optinal break line
 * str; //start state
 * str; //end state
 * str; //end state
 * str; //end state
 * s usize; /number of bands
 * str (char )+ > \(char, Direction\)+ str; // function: the state -> chars that should be in the bands -> literal '>' -> sequence of actions for each band (char to write, where to move) -> next_state
 *
 * example:
 * q0;
 * q1;
 * s 1;
 * q0 0 > (1, R) q0;
 * q0 1 > (1, R) q0;
 * q0 _ > (_, N) q1;
 */
impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s
            .split(';')
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
        todo!()
    }
}
