use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

type Link<T> = Option<Rc<ListElement<T>>>;

struct ListElement<T> where T: Debug {
    value: T,
    next: Link<T>,
}

struct LinkedList<T> where T: Debug {
    head: Link<T>,
}

impl <T> LinkedList<T> where T: Debug {

    pub fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: T) {

        let new_node = ListElement {
            value,
            next: self.head.clone()
        };
    
        self.head = Some(Rc::new(new_node));     
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next.clone();
            Rc::try_unwrap(x).ok().map(|x| x.value)
        }).flatten()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(Rc::borrow) }
    }


    fn print(&mut self) {
        let mut curr = &self.head;
        while let Some(ListElement {value, next}) = curr.as_ref().map(Rc::borrow)  {
            print!("{:?}", value);
            curr = &next;
        }
        println!()
    }
}

pub struct Iter<'a, T> where T: Debug {
    next: Option<&'a ListElement<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> where T: Debug {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(Rc::borrow);
            &node.value
        })
    }
}

fn main() {
    let mut list = LinkedList::new();
    
    list.push("Hello");
    list.push("how");
    list.push("are");
    list.push("so");
    let remove = list.pop();
    println!("remove item: {:?}", remove);
    list.push("your");

    list.print();
    println!("Second run ...");
    list.print();

    for item in list.iter() {
        println!("{}", item);
    }
}
