use rand::rng;
use rand_distr::{Exp, Distribution};

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: Vec<usize>,
    pub to: Vec<usize>,
    pub firing_time: f64,
}

impl Transition {
    pub fn new(from: Vec<usize>, to: Vec<usize>) -> Transition {
        Transition {
            from,
            to,
            firing_time: -1.0,
        }
    }

    pub fn new_firing_time(&mut self) {
        let lambda = 1.0;
        let exp_dist = Exp::new(lambda).unwrap();
    
        let mut rng = rng();
    
        let sample = exp_dist.sample(&mut rng);
        
        self.firing_time = sample;
    }
}