struct TransitionFunction<'a> {
    origin: &'a str,
    bands_requirements: Vec<char>,
    bands_actions: Vec<(char, &'a str)>,
}

impl<'a> TransitionFunction<'a> {
    fn new(
        origin: &'a str,
        bands_requirements: Vec<char>,
        bands_actions: Vec<(char, &'a str)>,
    ) -> Self {
        if bands_requirements.len() != bands_actions.len() {
            //TODO message
            panic!("invalid bands");
        }
        Self {
            origin,
            bands_requirements,
            bands_actions,
        }
    }
}

struct Machine<'a> {
    alphabet: Vec<char>,
    size: usize,
    states_names: Vec<&'a str>,
    start_state: &'a str,
    end_state: &'a str,
    transition_functions: Vec<&'a TransitionFunction<'a>>,
}
impl<'a> Machine<'a> {
    fn new(
        alphabet: Vec<char>,
        size: usize,
        states: Vec<&'a str>,
        start_state: &'a str,
        end_state: &'a str,
        transition_functions: Vec<&'a TransitionFunction<'a>>,
    ) -> Result<Self, &'static str> {
        //TODO make this more idiomatic? and refactor
        //TODO make sure the machine has a way to end
        for f in transition_functions.iter() {
            if !states.contains(&f.origin) {
                return Err("origin not in states");
            }
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
            for action in f.bands_actions.iter() {
                if !alphabet.contains(&action.0) {
                    return Err("character not defined in alphabet");
                }
                if !states.contains(&action.1) {
                    return Err("origin not in states");
                }
            }
        }
        Ok(Self {
            alphabet,
            size,
            states_names: states,
            start_state,
            end_state,
            transition_functions,
        })
    }
}
struct Band {
    data: Vec<char>,
}
impl Band {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

trait Accept<T> {
    type Result;
    fn accept(&self, value: T) -> Self::Result;
}
struct MachineExecutor<'a> {
    machine: &'a Machine<'a>,
    bands: Vec<Band>,
}

impl<'a> MachineExecutor<'a> {
    fn new(machine: &'a Machine<'a>) -> Self {
        let mut bands = Vec::new();
        for _ in 0..machine.size {
            bands.push(Band::new());
        }
        Self { machine, bands }
    }
}
impl Accept<char> for MachineExecutor<'_> {
    type Result = &'static str;

    fn accept(&self, value: char) -> Self::Result {
        todo!()
    }
}

impl Accept<String> for MachineExecutor<'_> {
    type Result = &'static str;

    fn accept(&self, value: String) -> Self::Result {
        todo!()
    }
}

fn create_simple_machine() {
    let machine = Machine::new(vec![], 1, vec![], "", "", vec![]).unwrap();
    let machine_executor = MachineExecutor::new(&machine);
}
