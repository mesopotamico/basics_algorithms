pub fn mergesort(vector: &mut Vec<i32>, start: usize , end: usize) {
    if start < end {
        let middle: usize = (start + end ) / 2; 
        mergesort(vector,start, middle);
        mergesort(vector,middle + 1, end);
        merge(vector,0, middle, vector.len() -1);
    }
}

pub fn merge(vector: &mut Vec<i32>, start:usize, middle: usize, end:usize){  
    let mut aux: Vec<i32> = vec![0;vector.len()];
   

    let mut index = 0;
    let mut left_ = start;
    let mut right_ = middle + 1;
    while left_ <= middle && right_ <= end{ 
        if vector[left_] <= vector[right_] {
            aux[index] = vector[left_];
            left_ += 1;
            index += 1;
        }
        if vector[right_] < vector[left_]{
            aux[index] = vector[right_];
            right_ += 1;
            index += 1;
        }
    }


    for k in right_..=end {
        aux[index] = vector[k];
        index += 1;
    }

    for k in left_..=middle {
        aux[index] = vector[k];
        index += 1;
    }

    for i in 0..vector.len(){
        vector[i] = aux[i];
    }

}
