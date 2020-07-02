use std::convert::TryInto;
use std::cmp::PartialEq;

struct LinkedList<T>{
    len:u32,
    node: Option<Box<Node<T>>>,
}

struct Node<T>{
    data:T,
    next: Option<Box<Node<T>>>,
}
impl <T> Node<T>{
    fn new(_data:T)->Node<T>{
        Node{
            data:_data,
            next:None,
        }
    }
}
impl<T:PartialEq> LinkedList<T>{
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
        cur.next = Option::Some(Box::new(newnode));
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
        cur.next = Option::Some(Box::new(newnode));
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
            println!("{}", position);
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
    //#TODO
    fn delete_all(&mut self, _data:T){
        let v = self.find_all(_data);
        let mut num = 0;
        for index in v{
            self.delete(index - num);
            num += 1;
        }
    }

    fn find_one(&mut self, _data:T)->i32{
        let mut cur = self.node.as_mut().unwrap().as_mut();
        let mut index = 0;
        loop{
            if cur.data==_data{
                return index;
            }
            if !cur.next.is_some(){
                return -1;
            }
            cur = cur.next.as_mut().unwrap().as_mut();
            index += 1;
        }
    }
    fn find_all(&mut self, _data:T)->Vec<u32>{
        let mut cur = self.node.as_mut().unwrap().as_mut();
        let mut index = 0;
        let mut v = Vec::new();
        loop{
            if cur.data==_data{
                v.push(index);
            }
            if !cur.next.is_some(){
                return v;
            }
            cur = cur.next.as_mut().unwrap().as_mut();
            index += 1;
        }
    }
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
    test();
}

fn test(){
    let mut head = LinkedList::new(8);
    head.push_back(10);
    head.push_back(20);
    head.push_back(30);

    head.push(15, -1);
    head.push(-100, 1);
    head.push_back(50);
    head.push(1,0);
    head.push_back(15);
    let v = head.as_vec();
    println!("{:?}", v);
    head.delete(0);
    head.push_back(15);
    let v = head.as_vec();
    println!("{:?}", v);
    head.delete_all(15);
    println!("{:?}", head.as_vec());
}