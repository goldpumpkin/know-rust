use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().cloned()
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let node3 = Rc::new(Node::new(3));

    node2.update_downstream(node3);
    node1.update_downstream(Rc::new(node2));

    // 如果 node2 还想被作为 另一个 Node 的 downstream
    let mut node4 = Node::new(4);
    node4.update_downstream(node1.get_downstream().unwrap());

    println!("node1:{:?}", node1);
    println!("node4:{:?}", node4);

    // 但是此时我们无法修改 DAG 里面的数据，怎么办呢？
}