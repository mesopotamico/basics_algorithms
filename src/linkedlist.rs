struct Node{
    data: i32,
    next: Option<Box<Node>>,
} 

pub struct LinkedList{
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { 
            head: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            data: value,
            next: None,
        });
        
        if self.head.is_none() {
            self.head = Some(new_node);
        }
        else {
            let mut current = self.head.as_mut();
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = node.next.as_mut();
            }
        }
    }
    pub fn display(&self) {
        let mut current = &self.head;
        while let Some(ref node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}
