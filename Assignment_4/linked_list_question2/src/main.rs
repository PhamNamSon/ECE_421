use self::LinkedList::*;
use im::list::*;

#[derive(Debug, PartialEq)]
pub enum LinkedList<T> {
    Tail,
    Head(List<T>),
}

impl<T: Clone> LinkedList<T> {
    pub fn new(item: T) -> Self {
        LinkedList::Head(cons(item, List::new()))
    }

    pub fn empty() -> Self {
        LinkedList::Tail
    }

    pub fn push(self, item: T) -> Self {
        match self {
            LinkedList::Head(list) => LinkedList::Head(cons(item, list)),
            LinkedList::Tail => LinkedList::new(item),
        }
    }

    pub fn push_back(&mut self, item: T) {
        *self = match self {
            LinkedList::Head(list) => LinkedList::Head(list.clone().append(List::from(vec![item]))),
            LinkedList::Tail => LinkedList::new(item),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l, Head(cons(4, cons(3, List::new()))));
        l.push_back(2);
        assert_eq!(l, Head(cons(4, cons(3, cons(2, List::new())))));
    }
}

fn main() {
}