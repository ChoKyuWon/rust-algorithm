use std::convert::TryInto;
struct LinkedList<T>{
    len:u32,
    node: Option<Box<Node<T>>>,
}

struct Node<T>{
    data:T,
    next: Option<Box<Node<T>>>
}
impl <T> Node<T>{
    fn new(_data:T)->Node<T>{
        Node{
            data:_data,
            next:None,
        }
    }
}
impl<T> LinkedList<T>{
    fn new(_data:T)->LinkedList<T>{
        LinkedList{
            len:1,
            node:Some(Box::new(Node::new(_data))),
        }
    }
    fn push_back(&mut self, _data:T){
        let newnode = Node{
            data: _data,
            next: None,
        };
        let mut cur = self.node.as_mut().unwrap().as_mut();
        while cur.next.is_some(){
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        cur.next = Option::Some(Box::<Node<T>>::new(newnode));
        self.len += 1;
    }
    
    fn push(&mut self, _data:T, position:i32){
        if position < 0{
            self.push_back(_data);
            return;
        }
        if self.len - 1 < position.try_into().unwrap(){
            panic!("OOB");
        }
        self.len += 1;
        if position == 0{
            let newnode = Node{
                data:_data,
                next: self.node.take(),
            };
            self.node = Some(Box::new(newnode));
            return;
        }
        let mut cur = self.node.as_mut().unwrap().as_mut();
        for _i in 0..position-1{
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let newnode = Node{
            data: _data,
            next : cur.next.take(),
        };
        cur.next = Option::Some(Box::<Node<T>>::new(newnode));
    }
    fn read(&mut self, position:u32)->&T{
        if self.len - 1 < position{
            panic!("OOB");
        }
        let mut cur = self.node.as_mut().unwrap().as_mut();
        for _i in 0..position{
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        return &cur.data;
    }
    fn delete(&mut self, position:u32){
        if self.len - 1 < position{
            panic!("OOB");
        }
        self.len -= 1;
        if position == 0{
            self.node = self.node.as_mut().unwrap().as_mut().next.take();
            return;
        }
        let mut cur = self.node.as_mut().unwrap().as_mut();
        for _i in 0..position-1{
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let drop = cur.next.as_mut().unwrap().as_mut();
        cur.next = drop.next.take();
    }
    // fn delete_all(&mut self, data:i32){}
    // fn find(&mut self, data:i32)->u32{return 0;}
    fn as_vec(&mut self)->Vec<&T>{
        let mut cur = self.node.as_mut().unwrap().as_mut();
        let mut v = Vec::new();
        loop {
            v.push(&cur.data);
            if !cur.next.is_some(){
                break;
            }
            else{
                cur = cur.next.as_mut().unwrap().as_mut();
            }
        }
        return v;
    }
}

fn main() {
    let s = String::from("hey");
    let mut head = LinkedList::new(s);
    let s2 = String::from("somebody");
    head.push_back(s2);
    let v = head.as_vec();
    println!("{:?}", v);
    // println!("{}", s);
}