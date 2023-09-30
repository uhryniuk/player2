pub trait Minimax<T: Clone, S: Clone + Ord> {
    /// Compares which value in all children is the best, max is true, then we max the first layer
    fn get_move(&self, depth: i32, mut max: bool) -> T {
        let mut result = self.value().clone();
        for _ in 1..(depth+1) {
            let children = self.generate_children(result, max);

            // NOTE Clean up this clone mayhem later...
            result = children.iter()
                .max_by_key(|v| {
                    self.evaluate(v.clone().clone())
                }).unwrap().clone();
            
            max = !max;  // Naught for the next layer.
        }

        result
    }
    fn value(&self) -> T;
    fn generate_children(&self, value: T, max: bool) -> Vec<T>;
    fn evaluate(&self, value: T) -> S;
}

