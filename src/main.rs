mod rand_vec;
use rand_vec::rand_vec;

mod doublelink;
use doublelink::DoubleLink;
//
// mod linkedlist;
// use linkedlist::LinkedList;


fn main() {

    let mut list = DoubleLink::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.display();

}


fn search_element(vector: &Vec<i32>, indice: usize) -> Option<&i32> {
    if indice < vector.len(){
        Some(&vector[indice])
    }
    else {
        None
    }
}

