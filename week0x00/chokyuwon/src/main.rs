#[derive(Clone)]
struct LinkedList<T>{
    data:T,
    next: Option<Box<LinkedList<T>>>
}

impl<T> LinkedList<T>{
    fn push_back(&mut self, _data:T){
        let newnode = LinkedList{
            data: _data,
            next: None,
        };
        let mut cur = self;
        while cur.next.is_some(){
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        cur.next = Option::Some(Box::<LinkedList<T>>::new(newnode));
    }
    fn push(&mut self, _data:T, position:i32){
        if position < 0{
            self.push_back(_data);
            return;
        }
        if position == 0{
            /*
            let newnode = LinkedList{
                data: _data,
                // TODO
                next : Option::Some(Box::new(*self)),
                //next:None,
            };
            *self = newnode;
            return;
            */
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
        cur.next = Option::Some(Box::<LinkedList<T>>::new(newnode));
    }
    fn read(&mut self, position:u32)->&T{
        let mut cur = self;
        for _i in 0..position{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        return &cur.data;
    }
    fn delete(&mut self, position:u32){
        let mut cur = self;
        for _i in 0..position-1{
            if cur.next.is_some() == false{
                panic!("OOB");
            }
            cur = cur.next.as_mut().unwrap().as_mut();
        }
        let drop = cur.next.as_mut().unwrap().as_mut();
        cur.next = drop.next.take();
    }
    fn delete_all(&mut self, data:i32){}
    fn find(&mut self, data:i32)->u32{return 0;}
    fn as_vec(&mut self)->Vec<&T>{
        let mut cur = self;
        let mut v = Vec::new();
        loop {
            v.push(&cur.data);
            //println!("{} ", cur.data);
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
    let v = head.as_vec();
    println!("{:?}", v);
    head.delete(1);
    let v = head.as_vec();
    println!("{:?}", v);

    let mut strlist = Box::new(LinkedList{
        data: "this",
        next: None,
    });
    strlist.push_back("is");
    strlist.push_back("a");
    let v = strlist.as_vec();
    println!("{:?}", v);
}