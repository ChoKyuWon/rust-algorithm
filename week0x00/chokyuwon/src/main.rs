use std::mem;

struct LinkedList<T>{
    len:u32,
    node: Node<T>,
}

struct Node<T>{
    data:T,
    next: Option<Box<Node<T>>>
}

impl<T> LinkedList<T>{
    fn push_back(&mut self, _data:T){
        let newnode = Node{
            data: _data,
            next: None,
        };
        let mut cur = &mut self.node;
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
        self.len += 1;
        let mut cur = &mut self.node;
        for _i in 0..position-1{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let mut newnode = Node{
            data: _data,
            next : cur.next.take(),
        };
        if position == 0{
            mem::swap(&mut cur.data, &mut newnode.data);
        }
        cur.next = Option::Some(Box::<Node<T>>::new(newnode));
    }
    fn read(&mut self, position:u32)->&T{
        let mut cur = &mut self.node;
        for _i in 0..position{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        return &cur.data;
    }
    fn delete(&mut self, position:u32){
        let mut cur = &mut self.node;
        for _i in 0..position-1{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let drop = cur.next.as_mut().unwrap().as_mut();
        cur.next = drop.next.take();
        self.len -= 1;
    }
    // fn delete_all(&mut self, data:i32){}
    // fn find(&mut self, data:i32)->u32{return 0;}
    fn as_vec(&mut self)->Vec<&T>{
        let mut cur = &mut self.node;
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
    let mut head = LinkedList{
        len:1,
        node: Node{
            data: 8,
            next: None,
        }
    };
    head.push_back(10);
    head.push_back(20);
    head.push_back(30);
    head.push(15, -1);
    head.push(-100, 1);
    head.push_back(50);
    head.push(1,0);
    let v = head.as_vec();
    println!("{:?}", v);
    head.delete(1);
    let v = head.as_vec();
    println!("{:?}", v);

    println!("{}", head.read(3));
    println!("{}", head.len);
    /*
    let mut strlist = Node{
        data: "this",
        next: None,
    };
    strlist.push_back("is");
    strlist.push_back("a");
    let v = strlist.as_vec();
    println!("{:?}", v);
    strlist.delete(2);
    strlist.push("alomost done", 0);
    let v = strlist.as_vec();
    println!("{:?}", v);
    */
}