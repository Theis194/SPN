use SPN::{PetriNet, Transition};

fn main() {
    let places: Vec<u32> = vec![2,0];
    let transitions: Vec<Transition> = vec![
        Transition {
            from: vec![0],
            to: vec![1],
            firing_time: -1.0,
        },
        Transition {
            from: vec![1],
            to: vec![0],
            firing_time: -1.0,
        },
    ];

    let mut petri_net = PetriNet::new();
    petri_net
        .add_places(places)
        .add_transitions(transitions);

    for _ in 0..3 {
        if petri_net.fire() {
            println!("fired: {:?}", petri_net)
        } else {
            println!("failed")
        }
    }
}