#[derive(Clone)]
struct LinkedList{
    data:i32,
    next: Option<Box<LinkedList>>
}

impl LinkedList{
    fn push_back(&mut self, _data:i32){
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
    fn push(&mut self, _data:i32, position:i32){
        if position < 0{
            self.push_back(_data);
            return;
        }
        if position == 0{
            let newnode = LinkedList{
                data: _data,
                // TODO
                next : Option::Some(Box::new(*self)),
                //next:None,
            };
            *self = newnode;
            return;
        }
        let mut cur = self;
        for _i in 0..position-1{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let newnode = LinkedList{
            data: _data,
            // TODO
            //next: Option::Some(Box::from_raw(Box::into_raw(cur.next.as_ref().unwrap().as_ref()))),
            next : cur.next.take(),
        };
        cur.next = Option::Some(Box::<LinkedList>::new(newnode));
    }
    fn read(&mut self, position:u32)->i32{
        let mut cur = self;
        for _i in 0..position{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        return cur.data;
    }
    // TODO
    // fn del(&self, position:u32){}
    fn print(&mut self){
        let mut cur = self;
        loop {
            println!("{} ", cur.data);
            if !cur.next.is_some(){
                break;
            }
            else{
                cur = cur.next.as_mut().unwrap().as_mut();
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
    head.push(15, -1);
    head.push(-100, 1);
    head.push_back(50);
    head.push(0,1);
    head.print();
    head.push(100,0);
    println!("-----------------");
    head.print();
}
