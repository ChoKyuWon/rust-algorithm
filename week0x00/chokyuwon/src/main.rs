// enum NextNode{
//     Node(Box<LinkedList>),
//     Nil,
// }
// impl NextNode{
//     fn node(&self){
//         /*
//         match(self){
//             Nil => 0,
//             Node => Node.Box::<LinkedList>.data,
//         }*/
//     }
// }

struct LinkedList{
    data:u32,
    next: Option<Box<LinkedList>>
}

impl LinkedList{
    fn push_back(&mut self, _data:u32){
        let newnode = LinkedList{
            data: _data,
            next: None,
        };
        self.next = Option::Some(Box::<LinkedList>::new(newnode));
    }
    // fn push_arbitary(&self, data:u32, position:u32){}
    // fn read(&self, position:u32){}
    // fn del(&self, position:u32){}
}

fn main() {
    let mut head = LinkedList{
        data: 8,
        next: None,
    };
    head.push_back(10);
    println!("{}, {}", head.data, head.next.unwrap().data);
}
