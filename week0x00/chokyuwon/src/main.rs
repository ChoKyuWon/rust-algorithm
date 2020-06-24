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
        let mut cur = self;
        while cur.next.is_some(){
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        cur.next = Option::Some(Box::<LinkedList>::new(newnode));
    }
    fn print(&mut self){
        let mut cur = self;
        loop {
            println!("{} ", cur.data);
            cur = cur.next.as_mut().unwrap().as_mut();
            if !cur.next.is_some(){
                println!("{} ", cur.data);
                break;
            }
        }
    }
    // fn push_arbitary(&self, data:u32, position:u32){}
    // fn read(&self, position:u32){}
    // fn del(&self, position:u32){}
}

fn main() {
    let mut head = Box::new(LinkedList{
        data: 8,
        next: None,
    });
    head.push_back(10);
    head.push_back(20);
    head.push_back(30);
    head.push_back(50);
    head.print();
    //println!("{}, {}", head.data, head.next.unwrap().data);
}
