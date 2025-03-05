use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;
use clap::Parser;
use num_cpus;
use SPN::{ PetriNet, Transition };

#[derive(Parser, Debug)]
struct Args {
    // Amount of simulations to run
    simulations: u64,

    // Optional amount of threads to use
    #[clap(short, long, default_value_t = num_cpus::get() as u64)]
    threads: u64,
}

fn main() {
    let args: Args = Args::parse();

    let num_threads = args.threads as u64;
    let simulations_per_thread = args.simulations / num_threads as u64;
    let remainder = args.simulations % num_threads as u64;

    let start = Instant::now();
    let mut handles = vec![];

    let shared_deadlock_flag = Arc::new(Mutex::new(false));

    for i in 0..num_threads {
        let deadlock_flag = Arc::clone(&shared_deadlock_flag);

        let thread_simulations = if i == 0 {
            simulations_per_thread + remainder
        } else {
            simulations_per_thread
        };

        let handle = thread::spawn(move || {
            // Places: P0 = Producer, P1 = Buffer, P2 = Consumer
            let places = vec![1, 1, 0];
        
            // Transitions: T0 (produce), T1 (consume), T2 (reset)
            let transitions = vec![
                Transition::new(vec![0], vec![1]), // T0: Producer to Buffer
                Transition::new(vec![1], vec![2]), // T1: Buffer to Consumer
                Transition::new(vec![2], vec![0]) // T2: Consumer resets Producer
            ];
        
            let mut petri_net = PetriNet::new();
            petri_net.add_places(places).add_transitions(transitions);

            for _ in 0..thread_simulations {
                let mut flag = deadlock_flag.lock().unwrap();

                if *flag {
                    break;
                }
 
                if !petri_net.fire() {
                    *flag = true;
                    break; // Stop on deadlock
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Simulation took: {:?} ms", duration.as_secs_f64() * 1000.0);
}
