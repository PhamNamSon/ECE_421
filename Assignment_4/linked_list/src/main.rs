#[derive(Debug, PartialEq)]

pub enum LinkedList<T> {
	Tail,
	Head(T,Box<LinkedList<T>>),
}
use self::LinkedList::*;

impl<T> LinkedList<T> {
	pub fn new(t:T) -> Self {
		Head(t,Box::new(Tail))
	}

	pub fn empty() -> Self {
		Tail
	}

	pub fn push(self,t:T) -> Self {
		Head(t,Box::new(self))
	}

	pub fn push_back(&mut self,t:T) {
		let mut current = self;
		loop {
			match current {
				&mut Head(_,ref mut next) => {
					current = next;
				}
				&mut Tail => {
					*current = Head(t,Box::new(Tail));
					break;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let mut l = LinkedList::new(3);
		l= l.push(4);
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Tail)))));

		l.push_back(2);
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Head(2,Box::new(Tail)))))));


	}
}
fn main() {

}