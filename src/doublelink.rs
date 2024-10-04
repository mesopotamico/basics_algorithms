struct Node {
    data: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

pub struct DoubleLink {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
} 

impl DoubleLink {
    pub fn new() -> Self {
        DoubleLink {
            head: None,
            tail: None,
        } 
    }

    pub fn insert(&mut self,value: i32) {
        let mut new_node = Box::new(Node{
            data: value,
            next: None,
            prev: None,
        });

        if self.head.is_none(){
            self.head = Some(new_node);
        }
        else { 
            let mut old_head = self.head.take().unwrap(); // Tomamos el antiguo cabeza
            old_head.next = Some(new_node); //Antigua cabeza siguiente es nuevo nodo
            let mut new_node = old_head.next.take().unwrap(); // Tomamos el antiguo cabeza
            new_node.prev = Some(old_head); // Nuevo nodo prev apunta a la antigua cabeza
            self.head = Some(new_node); // Actualizamos la cabeza
        }
    }

    pub fn display(&mut self) {

        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.prev;
        }
        print!("None");

    }
}


