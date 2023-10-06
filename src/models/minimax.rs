use rand::Rng;
pub trait Minimax<T: Clone, S: Clone + Ord> {
    /// Compares which value in all children is the best, max is true, then we max the first layer
    fn get_move(&self, depth: i32, max: bool) -> T {

        let mut computed: Vec<(T, S)> = Vec::new();
        let possible = self.generate_children(self.value(), max);

        for child in possible {
            let mut local_max = !max.clone(); // TODO double check if we should naught it here?
            let mut result = child.clone();
            for _ in 0..(depth-1) { // TODO fix the depth, think about what it should be.
                let local = self.generate_children(result.clone(), local_max);
                
                
                let mut local_optimal = result.clone();
                for child in local.iter() {
                    local_optimal = if local_max { self.max(child.clone(), result.clone()) } else { self.min(child.clone(), result.clone()) }
                }
                result = local_optimal;

                local_max = !local_max;
            }
            computed.push((child, self.evaluate(result)));
        }

        let mut optimal: T = self.value();
        let optimal_val = self.evaluate(optimal.clone());
        for (t, _) in computed.iter() { 
            optimal = if max { self.max(t.clone(), optimal.clone()) } else { self.min(t.clone(), optimal.clone()) }
        }
        optimal = if optimal_val == self.evaluate(optimal.clone()) {
            let mut rng = rand::thread_rng();
            let (t,s) = computed.iter().nth(rng.gen_range(0..7)).unwrap().clone();
            t
        } else { optimal.clone() };


        optimal
    }
    
    fn value(&self) -> T;
    fn max(&self, x: T, y: T ) -> T {
        if self.evaluate(x.clone()) > self.evaluate(y.clone()) { x.clone() } else { y.clone() }
    }
    fn min(&self, x: T, y: T ) -> T {
        if self.evaluate(x.clone()) < self.evaluate(y.clone()) { x.clone() } else { y.clone() }
    }
    fn generate_children(&self, value: T, max: bool) -> Vec<T>;
    fn evaluate(&self, value: T) -> S;
}

