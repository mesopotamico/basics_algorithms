// Min heap
pub struct Heap{
    vector: Vec<i32>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            vector: (0..100).map(|x| x % 2).collect(), 
        }

    }

    pub fn add(&mut self, number: i32) {
        // ir revisando indices 2i y 2i + 1

        let vector = &mut self.vector;
        if vector.is_empty(){
            vector[1] = 0;
        }
        for i in 0..vector.len() - 1{
            let clone_value = vector[i].clone();
            if vector[i] < vector[2 * i]  {
                vector[2 * i] = clone_value; 
            } 
        }
    }
    
    pub fn display(&self) {
    
        for i in &self.vector{
            println!("{}", i);
        } 
    }


}
