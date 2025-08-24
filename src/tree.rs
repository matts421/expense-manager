use std::cell::RefCell;
use std::io;
use std::rc::Rc;

pub struct Node {
    action: fn() -> io::Result<usize>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(action: fn() -> io::Result<usize>) -> Rc<Node> {
        Rc::new(Node {
            action,
            children: RefCell::new(Vec::new()),
        })
    }

    pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        parent.children.borrow_mut().push(Rc::clone(child));
    }

    pub fn run(self: &Rc<Self>) -> io::Result<()> {
        let index = (self.action)()?;
        let children = self.children.borrow();

        if let Some(child) = children.get(index) {
            child.run()?;
        }
        Ok(())
    }
}
