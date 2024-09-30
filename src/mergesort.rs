pub fn mergesort(vector: &mut Vec<i32>) {

}

pub fn merge(vector: &mut Vec<i32>, start:usize, middle: usize, end:usize) {
   
    let mut index = 0;
    let mut right_ = 0;
    let mut left_ = 0;
    let mut aux: Vec<i32>;
    while right_ <= middle && left_ <= end{ 
        if vector[left_] <= vector[right_]{
            aux[index] = vector[left_];
            left_ += 1;
        }
        if vector[right_] < vector[left_]{
            aux[index] = vector[right_];
            right_ += 1;
        }
        index += 1;
    }
    if right_ == middle {
        //Aca hay un problema

    }

}
