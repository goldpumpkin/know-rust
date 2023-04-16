use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().cloned()
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    node2.update_downstream(Rc::new(RefCell::new(node3)));
    node1.update_downstream(Rc::new(RefCell::new(node2)));

    // 如果 node2 还想被作为 另一个 Node 的 downstream
    let mut node4 = Node::new(4);
    let rc = node1.get_downstream().unwrap();
    node4.update_downstream(rc);

    println!("node1:{:?}", node1);
    println!("node4:{:?}", node4);

    // 如果想修改 node2 关联的 downstream
    let node5 = Node::new(5);
    let node2 = node4.get_downstream().unwrap();
    node2.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));
    println!("node2:{:?}", node2);
}