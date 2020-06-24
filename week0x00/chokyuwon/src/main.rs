enum NextNode{
    Node(Box<LinkedList>),
    Nil,
}
impl NextNode{
    fn node(&self){
    }
}

struct LinkedList{
    data:u32,
    //next:Box<LinkedList>,
    next: NextNode
}

impl LinkedList{
    fn push_back(&self, data:u32){}
    fn push_arbitary(&self, data:u32, position:u32){}
    fn read(&self, position:u32){}
    fn del(&self, position:u32){}
}

fn main() {
    let newnode = LinkedList{
        data: 10,
        next: NextNode::Nil,
    };
    let head = LinkedList{
        data: 8,
        //next: NextNode::Node(Box::<LinkedList>::new(newnode)),
        next: NextNode::Nil,
    };
    println!("{}, {}", head.data, newnode.data);
}
