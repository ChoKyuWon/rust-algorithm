use std::io::{self, Write};

struct Node {
    value: i32,
    prev: ,// Node pointer
    next: // Node pointer
}

struct LinkedList {
    tail: //Node pointer
    len: u32
}

impl LinkedList {
    fn insert(&mut self, input: i32) {
    }

    fn delete(&mut self, input: i32) {
    }

    fn print_list(&self) {
    }
}

fn make_linkedlist() -> mut LinkedList {
    let mut dummy_node = Node { value: 0, prev: , next: };
    dummy_node.prev = ;
    dummy_node.next = ;
    LinkedList {
        tail: dummy_node,
        len: 0
    }
}

fn main() {
    let mut list = make_linkedlist();
    
    println!("This is circular doubly linked list, dude!");
    
    loop {
        println!("0: exit, 1: insert, 2: delete, 3: print");
        print!("command: ");
        io::stdout().flush().unwrap();
        
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
                    print!("Delete number: ");
                }
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("input error");
                let input: i32 = input.trim().parse().expect("invalid input");

                if oper == 1 {
                    println!("insert {}", input);
                    list.insert(input);
                }
                else {
                    println!("delete {}", input);
                    list.delete(input);
                }
            },
            3 => println!("print list"), //list.print_list(),
            _ => println!("Invalid input"),
        }
    }
    println!("Good Bye");
}
