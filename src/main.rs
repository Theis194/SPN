use std::time::Instant;
use SPN::{PetriNet, Transition};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Amount of simulations to run
    simulations: u64,
}

fn main() {
    let args: Args = Args::parse();

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

    for _ in 0..args.simulations {
        if !petri_net.fire() {
            break; // Stop on deadlock
        }
    }

    let duration = start.elapsed();
    println!("{:?}", duration.as_secs_f64() * 1000.0);
}
