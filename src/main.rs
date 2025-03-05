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
    let num_threads = num_cpus::get();
    let simulations_per_thread = args.simulations / num_threads as u64;
    
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
    println!("Simulation took: {:?} ms", duration.as_secs_f64() * 1000.0);
    println!("Failed simulations: {}", total_failures);
}