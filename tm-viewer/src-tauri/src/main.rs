#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use app::tm::SimulationStep;
use compiler::tm::{Direction, Machine, State, TransitionFunction};
use tauri::Manager;

#[derive(Default)]
struct AppState {
    machine: Mutex<Option<Machine>>,
}

#[tauri::command]
fn get_test_machine(state: tauri::State<'_, AppState>) -> Machine {
    let mut states = HashMap::new();
    states.insert(
        "q0".to_string(),
        State::new(
            vec![
                TransitionFunction::new(
                    "q0".to_string(),
                    vec!['0'.to_string()],
                    vec![('1'.to_string(), Direction::Right)],
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
            false,
        ),
    );
    states.insert("q1".to_string(), State::new(vec![], true));
    let machine = Machine::new(
        HashSet::from(['0'.to_string(), '1'.to_string()]),
        1,
        states,
        "q0".to_string(),
    )
    .unwrap();
    *state.machine.lock().unwrap() = Some(machine.clone());
    machine
    //MachineExecutor::new(machine, "000".chars().collect())
}

#[tauri::command]
fn accept_input(input: String, state: tauri::State<'_, AppState>) -> Vec<SimulationStep> {
    println!("{:?}", state.machine.lock().unwrap());
    vec![]
}

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![get_test_machine, accept_input])
        .setup(|app_handle| {
            app_handle.manage(AppState::default());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
