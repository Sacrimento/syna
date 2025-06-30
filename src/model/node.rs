use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    uuid: Uuid,
    pub children: Vec<NodeRef>,
    pub neighbors: HashSet<Uuid>,
}

impl Node {
    pub fn new() -> NodeRef {
        Rc::new(RefCell::new(Self {
            uuid: Uuid::new_v4(),
            children: vec![],
            neighbors: HashSet::new(),
        }))
    }

    pub fn add_child(&mut self, child: NodeRef) {
        self.children.push(child);
    }

    pub fn add_neighbor(&mut self, neighbor: &NodeRef, bidirectional: bool) {
        let nuuid = neighbor.borrow().uuid();

        self.neighbors.insert(nuuid);
        if bidirectional {
            neighbor.borrow_mut().neighbors.insert(self.uuid);
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}
