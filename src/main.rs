use std::time::Instant;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use clap::Parser;
use num_cpus;
use SPN::{ PetriNet, Transition };

#[derive(Parser, Debug)]
struct Args {
    // Amount of simulations to run
    simulations: u64,
}

fn main() {
    let args: Args = Args::parse();
    
    let duration = parallel_simulation(args.simulations);

    println!("Simulation took: {:?} ms", duration.0);
    println!("Failed simulations: {}", duration.1);
}

fn sequential_simulation(simulations: u64) -> (f64, usize) {
    // Places: P0 = Producer, P1 = Buffer, P2 = Consumer
    let places = vec![1, 1, 0];

    // Transitions: T0 (produce), T1 (consume), T2 (reset)
    let transitions = vec![
        Transition::new(vec![0], vec![1]), // T0: Producer to Buffer
        Transition::new(vec![1], vec![2]), // T1: Buffer to Consumer
        Transition::new(vec![2], vec![0]), // T2: Consumer resets Producer
    ];

    let mut petri_net = PetriNet::new();
    petri_net
        .add_places(places)
        .add_transitions(transitions);

    let start = Instant::now();

    for _ in 0..simulations {
        if !petri_net.fire() {
            break; // Stop on deadlock
        }
    }

    (start.elapsed().as_secs_f64() * 1000.0, 0)
}

fn parallel_simulation(simulations: u64) -> (f64, usize) {
    let num_threads = num_cpus::get();
    let simulations_per_thread = simulations / num_threads as u64;
    
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut failures = 0;
            let mut petri_net = PetriNet::new();
            petri_net.add_places(vec![1, 1, 0])
                     .add_transitions(vec![
                         Transition::new(vec![0], vec![1]),
                         Transition::new(vec![1], vec![2]),
                         Transition::new(vec![2], vec![0]),
                     ]);

            for _ in 0..simulations_per_thread {
                if !petri_net.fire() {
                    failures += 1;
                }
            }
            failures
        });

        handles.push(handle);
    }

    let total_failures: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();

    let duration = start.elapsed();

    (duration.as_secs_f64() * 1000.0, total_failures)
}