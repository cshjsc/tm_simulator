use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Right,
    Left,
    Unchanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionFunction {
    origin: String,
    bands_requirements: Vec<String>,
    bands_actions: Vec<(String, Direction)>,
    next_state_name: String,
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
    transition_functions: Vec<TransitionFunction>,
    is_end_state: bool,
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
    alphabet: HashSet<String>,
    size: usize,
    states: HashMap<String, State>,
    start_state_name: String,
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

#[derive(Serialize)]
pub struct MachineExecutor {
    machine: Machine,
    bands: Vec<Vec<String>>,
    bands_cursors: Vec<usize>,
    current_state: State,
}

impl MachineExecutor {
    pub fn new(machine: Machine, input: Vec<String>) -> Self {
        let mut bands = vec![input];
        bands.resize_with(machine.size, Vec::new);
        //for _ in 1..machine.size {
        //    bands.push(Vec::new());
        //}
        let start_state = machine
            .states
            .get(&machine.start_state_name)
            .unwrap()
            .clone();
        let number_of_bands = machine.size;
        Self {
            machine,
            bands,
            bands_cursors: vec![0; number_of_bands],
            current_state: start_state,
        }
    }
}

impl MachineExecutor {
    pub fn next_step(&mut self) -> Option<TransitionFunction> {
        let current_state = self.current_state.clone();
        if let Some(transition) = current_state
            .transition_functions
            .iter()
            .find(|f| self.function_matches_band(f))
        {
            self.apply_transition_to_band(transition);
            return Some(transition.clone());
        }
        None
    }

    fn apply_transition_to_band(&mut self, transition: &TransitionFunction) {
        for x in 0..self.machine.size {
            let band_cursor = self.bands_cursors.get(x).unwrap();
            let band = self.bands.get_mut(x).unwrap();
            band[*band_cursor] = transition.bands_actions.get(x).unwrap().0.to_string();
            self.bands_cursors[x] = match transition.bands_actions.get(x).unwrap().1 {
                Direction::Right => {
                    if band_cursor + 1 == band.len() {
                        band.push("_".to_string());
                    }
                    band_cursor + 1
                }
                Direction::Left => {
                    if *band_cursor == 0 {
                        *band_cursor
                    } else {
                        band_cursor - 1
                    }
                }
                Direction::Unchanged => *band_cursor,
            };
        }
        self.current_state = self
            .machine
            .states
            .get(&transition.next_state_name)
            .unwrap()
            .clone();
    }

    fn function_matches_band(&self, transition: &TransitionFunction) -> bool {
        for x in 0..self.machine.size {
            let band_cursor = self.bands_cursors.get(x).unwrap();
            let band_char = self.bands.get(x).unwrap().get(*band_cursor).unwrap();
            let transition_char = transition.bands_requirements.get(x).unwrap();
            if band_char == transition_char {
                return true;
            }
        }
        false
    }
}
#[derive(Serialize)]
pub struct SimulationStep {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_machine() {
        let mut states = HashMap::new();
        states.insert(
            "q0".to_string(),
            State {
                transition_functions: vec![
                    TransitionFunction::new(
                        "q0".to_string(),
                        vec!["0".to_string()],
                        vec![("1".to_string(), Direction::Right)],
                        "q0".to_string(),
                    ),
                    TransitionFunction::new(
                        "q0".to_string(),
                        vec!['1'.to_string()],
                        vec![('1'.to_string(), Direction::Right)],
                        "q0".to_string(),
                    ),
                    TransitionFunction::new(
                        "q0".to_string(),
                        vec!['_'.to_string()],
                        vec![('_'.to_string(), Direction::Unchanged)],
                        "q1".to_string(),
                    ),
                ],
                is_end_state: false,
            },
        );
        states.insert(
            "q1".to_string(),
            State {
                transition_functions: vec![],
                is_end_state: true,
            },
        );
        let machine = Machine::new(
            HashSet::from(['0'.to_string(), '1'.to_string()]),
            1,
            states,
            "q0".to_string(),
        )
        .unwrap();
        let mut machine_executor = MachineExecutor::new(
            machine,
            vec!["0".to_string(), "0".to_string(), "0".to_string()],
        );
        println!("initial state{:?}", machine_executor.bands);
        while let Some(step) = machine_executor.next_step() {
            println!("{:?}", step);
            println!(" after first step");
            println!("{:?}", machine_executor.bands);
            println!("{:?}", machine_executor.bands_cursors);
            println!("{:?}", machine_executor.current_state);
        }
        assert!(machine_executor.current_state.is_end_state);
        let bands = machine_executor.bands.get(0).unwrap();
        assert_eq!(
            bands.iter().flat_map(|s| s.chars()).collect::<String>(),
            "111_"
        );
    }
}
