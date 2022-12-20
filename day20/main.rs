use aoc22::read;
use chumsky::prelude::*;
use std::{
	cell::RefCell,
	fmt::{self, Debug, Formatter},
	ops::Mul,
	rc::Rc
};

type T = i64;

fn parser() -> impl Parser<char, Vec<T>, Error = Simple<char>> {
	let digit = one_of(['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
		.repeated()
		.at_least(1)
		.collect::<String>()
		.map(|digits| digits.parse().unwrap());
	digit
		.then_ignore(one_of([' ', '\n']).repeated().at_least(1))
		.repeated()
		.at_least(1)
		.then_ignore(end())
}

#[derive(Clone, Copy)]
struct Entry {
	idx: usize,
	value: T
}

struct List {
	q: Vec<Rc<RefCell<Entry>>>,
	list: Vec<Rc<RefCell<Entry>>>
}

impl Debug for List {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("[")?;
		for entry in &self.list {
			let entry = entry.borrow();
			if entry.idx > 0 {
				f.write_str(", ")?;
			}
			write!(f, "{}", entry.value)?;
		}
		f.write_str("]")
	}
}

impl Mul<T> for List {
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		for entry in self.list.iter() {
			entry.borrow_mut().value *= rhs;
		}
		self
	}
}

impl List {
	fn new(input: &[T]) -> Self {
		let mut q = Vec::new();
		let mut list = Vec::new();
		for (idx, value) in input.iter().enumerate() {
			let entry = Rc::new(RefCell::new(Entry { idx, value: *value }));
			q.push(Rc::clone(&entry));
			list.push(entry);
		}
		Self { q, list }
	}

	fn mix(&mut self) {
		println!("mixing ...");
		for entry in &self.q {
			let entry: Entry = *entry.borrow();

			// println!("{} moves:", entry.value);
			let value = entry.value % (self.list.len() as i64 - 1);
			if value > 0 {
				for i in entry.idx .. entry.idx + value as usize {
					let idx = i % self.list.len();
					let next = (idx + 1) % self.list.len();
					self.list[idx].borrow_mut().idx = next;
					self.list[next].borrow_mut().idx = idx;
					self.list.swap(idx, next);
				}
			}
			if value < 0 {
				for i in (entry.idx as i64 + value .. entry.idx as i64).rev() {
					let idx = i.rem_euclid(self.list.len() as i64) as usize;
					let next = (idx + 1) % self.list.len();
					self.list[idx].borrow_mut().idx = next;
					self.list[next].borrow_mut().idx = idx;
					self.list.swap(idx, next);
				}
			}

			// println!("{:?}", self);
		}
	}

	fn sum(&self) -> T {
		let mut sum = 0;
		let idx = self
			.list
			.iter()
			.position(|entry| entry.borrow().value == 0)
			.unwrap();
		for i in [1000, 2000, 3000] {
			let entry = &self.list[(i + idx) % self.list.len()];
			sum += entry.borrow().value;
		}
		sum
	}
}

fn main() -> anyhow::Result<()> {
	let input = read("input.txt", parser())?;
	println!("Input has {} values", input.len());

	// part 1

	let mut list = List::new(&input);
	list.mix();
	println!("{}", list.sum());

	// part 2

	let mut list = List::new(&input) * 811589153;
	for _ in 0 .. 10 {
		list.mix()
	}
	println!("{}", list.sum());

	Ok(())
}
