use self::LinkedList::*;
use im::list::*;

#[derive(Debug, PartialEq)]
pub enum LinkedList<T> {
    Tail,
    Head(List<T>),
}

impl<T> LinkedList<T> {
    pub fn new(t: T) -> Self {
        LinkedList::Head(cons(t, List::new()))
    }

    pub fn empty() -> Self {
        LinkedList::Tail
    }

    pub fn push(self, t: T) -> Self {
        match self {
            LinkedList::Head(list) => LinkedList::Head(cons(t, list)),
            LinkedList::Tail => LinkedList::new(t),
        }
    }

    pub fn push_back(&mut self, t: T) {
        *self = match std::mem::replace(self, LinkedList::Tail) {
            LinkedList::Head(list) => LinkedList::Head(list.append(List::from(vec![t]))),
            LinkedList::Tail => LinkedList::new(t),
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