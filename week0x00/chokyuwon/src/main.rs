use std::borrow::Borrow;
use std::ops::Deref;

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
    /*
    fn push_arbitary(&mut self, _data:u32, position:u32){
        let mut cur = self;
        for _i in (0..position){
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let newnode = LinkedList{
            data: _data,
            //next: Option::Some(Box::<LinkedList>::new(cur.next.as_ref().unwrap().as_ref())),
            next : cur.next,
        };
        cur.next = Option::Some(Box::<LinkedList>::new(newnode));
    }
    */
    fn read(&mut self, position:u32)->u32{
        let mut cur = self;
        for _i in (0..position){
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        return cur.data;
    }
    // fn del(&self, position:u32){}
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
    println!("{}, {}", head.read(0), head.read(1));
    head.print();
}
