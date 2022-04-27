use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use compiler::tm::*;

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
