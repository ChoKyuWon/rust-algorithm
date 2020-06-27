struct LinkedList<T>{
    len:u32,
    node: Option<Box<Node<T>>>,
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
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let newnode = Node{
            data: _data,
            next : cur.next.take(),
        };
        cur.next = Option::Some(Box::<Node<T>>::new(newnode));
    }
    fn read(&mut self, position:u32)->&T{
        let mut cur = self.node.as_mut().unwrap().as_mut();
        for _i in 0..position{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        return &cur.data;
    }
    fn delete(&mut self, position:u32){
        if position == 0{
            self.node = self.node.as_mut().unwrap().as_mut().next.take();
            return;
        }
        let mut cur = self.node.as_mut().unwrap().as_mut();
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
    let mut head = LinkedList{
        len:1,
        node: Some(Box::new(Node{
            data: 8,
            next: None,
        })),
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
    head.delete(0);
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