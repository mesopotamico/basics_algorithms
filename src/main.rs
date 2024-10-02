mod rand_vec;
use rand_vec::rand_vec;

mod linkedlist;
use linkedlist::LinkedList;


fn main() {

    // let mut disorder = rand_vec(10);
    // println!("{:?}", disorder);
    //
    // let end: usize = disorder.len() - 1;
    // mergesort(&mut disorder,0, end);
    //
    // println!("Test mergesort: {:?}",disorder);
    //
    let mut list = LinkedList::new();
    list.insert(1);
    list.insert(2);
    list.insert(2);
    list.insert(1);
    list.insert(5);
    list.display();
    let random = rand_vec(10);
    println!("{:?}", random);
    match search_element(&random, 7) {
        None => println!("Index out of bounds"),
        Some(value) => println!("The value is: {}", value),

    }
}

fn search_element(vector: &Vec<i32>, indice: usize) -> Option<&i32> {
    if indice < vector.len(){
        Some(&vector[indice])
    }
    else {
        None
    }
}
