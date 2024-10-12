mod rand_vec;
use rand_vec::rand_vec;

// mod doublelink;
// use doublelink::DoubleLink;
//
// mod linkedlist;
// use linkedlist::LinkedList;

mod heap;
use heap::Heap;


fn main() {

    // let mut list = DoubleLink::new();
    // list.insert(1);
    // list.insert(2);
    let mut heap = Heap::new(); 
    heap.add(1);
    heap.add(2);
    heap.add(3);
    heap.add(4);
    heap.display();

}


fn search_element(vector: &Vec<i32>, indice: usize) -> Option<&i32> {
    if indice < vector.len(){
        Some(&vector[indice])
    }
    else {
        None
    }
}

