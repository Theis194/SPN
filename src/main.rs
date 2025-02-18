use rand::rng;
use rand_distr::{Exp, Distribution};

fn main() {
    let mut places: Vec<u32> = vec![2,0];
    let mut transitions: Vec<Transition> = vec![
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


    for transition in &mut transitions {
        let mut ready: bool = true;
        for &from in &transition.from {
            if places[from] == 0 {
                ready = false;
            }
        }

        if ready {
            transition.firing_time = new_firing_time();
        }
    }

    println!("Token distribution: {:?}", &places);
    for transition in &transitions {
        println!("{:?}", transition)
    }

    let mut next_transition: Option<&mut Transition> = None;
    for transition in &mut transitions {
        if transition.firing_time != -1.0 {
            match next_transition {
                Some(ref trans) => {
                    if trans.firing_time > transition.firing_time {
                        next_transition = Some(transition);
                    }
                },
                None => {
                    next_transition = Some(transition);
                }
            }
        }
    }

    match next_transition {
        Some(transition) => {
            println!("Lowest firing time {:?}", transition);

            for i in 0..transition.from.len() {
                places[transition.from[i]] -= 1;
            }
            for i in 0..transition.to.len() {
                places[transition.to[i]] += 1;
            }

            transition.firing_time = -1.0;
        },
        None => {
            println!("Failed to find transition");
        }
    }

    println!("Token distribution: {:?}", &places);
    for transition in &transitions {
        println!("{:?}", transition)
    }
}

fn new_firing_time() -> f64 {
    let lambda = 1.0;
    let exp_dist = Exp::new(lambda).unwrap();

    let mut rng = rng();

    let sample = exp_dist.sample(&mut rng);
    
    sample
}

#[derive(Debug, Clone)]
struct Transition {
    pub from: Vec<usize>,
    pub to: Vec<usize>,
    pub firing_time: f64,
}