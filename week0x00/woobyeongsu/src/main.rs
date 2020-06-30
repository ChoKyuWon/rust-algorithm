use std::io;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>
}

struct LinkedList {
    tail: Box<Node>,
    num: u32
}

impl LinkedList {
    fn insert(&self, input: i32) {
        let new_node = Node { value: input, next: (*self.tail).next, prev: Some(self.tail) };
        let new_node = Box::new(new_node);
        (*self.tail).next = Some(new_node);
        self.num += 1;
    }

    fn delete(&self, target: i32) {
        if self.num == 0 {
            println!("You tried to delete empty list.");
            return;
        }

        let mut cur = (*self.tail).next.take().unwrap();
        let mut idx = 0;
        while idx < self.num {
            if target != (*cur).value {
                cur = (*cur).next.take().unwrap();
                idx += 1;
                continue;
            }

            let prev = (*cur).prev.take().unwrap();
            let next = (*cur).next.take().unwrap();
            (*prev).next = (*cur).next;
            (*next).prev = (*cur).prev;
            drop(cur);
            self.num -= 1;

            return;
        }

        println!("Cannot find the target");
    }

    fn print_list(&self) {
        if self.num == 0 {
            println!("list is empty");
            return;
        }

        let mut cur = (*self.tail).next.take().unwrap();
        let mut idx = 0;
        println!("length of list : {}", self.num);
        while idx < self.num {
            print!("{} ", (*cur).value);
            idx += 1;
            cur = (*cur).next.take().unwrap();
        }
        println!("");
    }
}

fn make_linkedlist() -> LinkedList {
    let mut dummy_node = Node { value: 0, next: None, prev: None };
    let dummy_node = Box::new(dummy_node);
    (*dummy_node).next = Some(dummy_node);
    (*dummy_node).prev = Some(dummy_node);
    let new_list = Box::new(LinkedList { tail: dummy_node, num: 0 });
    *new_list
}

fn main() {
    let list: LinkedList = make_linkedlist();
    println!("This is circular doubly linked list, dude!");
    loop {
        println!("0: exit, 1: insert, 2: delete, 3: print");
        print!("command: ");
        let mut oper = String::new();
        io::stdin().read_line(&mut oper).expect("input error");

        let oper: i32 = oper.trim().parse().expect("invalid input");

        match oper {
            0 => break,
            1 | 2 => {
                if oper == 1 {
                    print!("Insert number: ");
                }
                else {
                    print!("Insert number: ");
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("input error");
                let input: i32 = input.trim().parse().expect("invalid input");

                if oper == 1 {
                    list.insert(input);
                }
                else {
                    list.delete(input);
                }
            },
            3 => list.print_list(),
            _ => println!("Invalid input"),
        }
    }

    println!("Good Bye");
}
