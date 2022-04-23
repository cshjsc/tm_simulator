use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Unchanged,
}

#[derive(Debug)]
pub struct TransitionFunction<'a> {
    origin: &'a str,
    bands_requirements: Vec<char>,
    bands_actions: Vec<(char, Direction)>,
    next_state_name: &'a str,
}

impl<'a> TransitionFunction<'a> {
    fn new(
        origin: &'a str,
        bands_requirements: Vec<char>,
        bands_actions: Vec<(char, Direction)>,
        next_state_name: &'a str,
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
#[derive(Debug)]
struct State<'a> {
    transition_functions: Vec<TransitionFunction<'a>>,
    is_end_state: bool,
}

struct Machine<'a> {
    alphabet: HashSet<char>,
    size: usize,
    states: HashMap<&'a str, State<'a>>,
    start_state_name: &'a str,
}
impl<'a> Machine<'a> {
    fn new(
        alphabet: HashSet<char>,
        size: usize,
        states: HashMap<&'a str, State<'a>>,
        start_state: &'a str,
        //end_state: &'a str,
        //transition_functions: Vec<&'a TransitionFunction<'a>>,
    ) -> Result<Self, &'static str> {
        //TODO make this more idiomatic? and refactor
        //TODO make sure the machine has a way to end
        let mut alphabet = alphabet;
        alphabet.insert(' ');
        if !states.contains_key(start_state) {
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

struct MachineExecutor<'a> {
    machine: &'a Machine<'a>,
    bands: Vec<Vec<char>>,
    bands_cursors: Vec<usize>,
    current_state: &'a State<'a>,
}

impl<'a> MachineExecutor<'a> {
    fn new(machine: &'a Machine<'a>, input: Vec<char>) -> Self {
        let mut bands = vec![input];
        for _ in 1..machine.size {
            bands.push(Vec::new());
        }
        Self {
            machine,
            bands,
            bands_cursors: vec![0; machine.size],
            current_state: machine.states.get(machine.start_state_name).unwrap(),
        }
    }
}

impl MachineExecutor<'_> {
    fn next_step(&mut self) -> Option<&TransitionFunction> {
        if let Some(transition) = self
            .current_state
            .transition_functions
            .iter()
            .find(|f| self.function_matches_band(f))
        {
            self.apply_transition_to_band(transition);
            return Some(transition);
        }
        None
    }

    fn apply_transition_to_band(&mut self, transition: &TransitionFunction) {
        for x in 0..self.machine.size {
            let band_cursor = self.bands_cursors.get(x).unwrap();
            let band = self.bands.get_mut(x).unwrap();
            band[*band_cursor] = transition.bands_actions.get(x).unwrap().0;
            self.bands_cursors[x] = match transition.bands_actions.get(x).unwrap().1 {
                Direction::Right => {
                    if band_cursor + 1 == band.len() {
                        band.push(' ');
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
        self.current_state = self.machine.states.get(transition.next_state_name).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_machine() {
        let mut states = HashMap::new();
        states.insert(
            "q0",
            State {
                transition_functions: vec![
                    TransitionFunction::new("q0", vec!['0'], vec![('1', Direction::Right)], "q0"),
                    TransitionFunction::new("q0", vec!['1'], vec![('1', Direction::Right)], "q0"),
                    TransitionFunction::new(
                        "q0",
                        vec![' '],
                        vec![(' ', Direction::Unchanged)],
                        "q1",
                    ),
                ],
                is_end_state: false,
            },
        );
        states.insert(
            "q1",
            State {
                transition_functions: vec![],
                is_end_state: true,
            },
        );
        let machine = Machine::new(HashSet::from(['0', '1']), 1, states, "q0").unwrap();
        let mut machine_executor = MachineExecutor::new(&machine, "000".chars().collect());
        println!("initial state{:?}", machine_executor.bands);
        while let Some(step) = machine_executor.next_step() {
            println!("{:?}", step);
            println!(" after first step");
            println!("{:?}", machine_executor.bands);
            println!("{:?}", machine_executor.bands_cursors);
            println!("{:?}", machine_executor.current_state);
        }
        assert!(machine_executor.current_state.is_end_state);
        assert_eq!(
            machine_executor.bands,
            vec!["111 ".chars().collect::<Vec<char>>()]
        );
    }
}
