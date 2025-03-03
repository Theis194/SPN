use super::transition::Transition;

#[derive(Debug, Clone)]
pub struct PetriNet {
    pub places: Vec<u32>,
    pub transitions: Vec<Transition>,
}

impl PetriNet {
    pub fn new() -> PetriNet {
        PetriNet {
            places: Vec::new(),
            transitions: Vec::new(),
        }
    }

    pub fn add_transitions(&mut self, transitions: Vec<Transition>) -> &mut Self {
        self.transitions = transitions;

        self
    }

    pub fn add_places(&mut self, places: Vec<u32>) -> &mut Self {
        self.places = places;

        self
    }

    pub fn fire(&mut self) -> bool {
        self.check_transitions();
        if let Some(transition_index) = self.get_next_transition_index() {
            let transition = &mut self.transitions[transition_index];
            //println!("Lowest firing time {:?}", transition);

            for &from in &transition.from {
                self.places[from] -= 1;
            }

            for &to in &transition.to {
                self.places[to] += 1;
            }

            transition.firing_time = -1.0;
            true
        } else {
            println!("Failed to find transiton");
            false
        }
    }

    pub fn check_transitions(&mut self) {
        for transition in &mut self.transitions {
            let ready = transition.from.iter().all(|&from| self.places[from] > 0);
    
            if ready {
                transition.new_firing_time();
            }
        }
    }

    pub fn get_next_transition_index(&self) -> Option<usize> {
        self.transitions
            .iter()
            .enumerate()
            .filter(|(_, transition)| transition.firing_time != -1.0)
            .min_by(|(_, a),(_,b)| a.firing_time.partial_cmp(&b.firing_time).unwrap())
            .map(|(index, _)| index)
    }
}