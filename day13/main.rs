use aoc22::read;
use chumsky::{prelude::*, text::digits};
use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Debug)]
pub enum Value {
	Number(u32),
	List(Vec<Value>)
}

impl Value {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		recursive(|parser| {
			choice((
				digits(10).map(|num: String| Self::Number(num.parse().unwrap())),
				just("[")
					.ignore_then(parser.clone())
					.then(just(",").ignore_then(parser).repeated())
					.map(|(first, mut list)| {
						list.insert(0, first);
						Self::List(list)
					})
					.then_ignore(just("]")),
				just("[]").map(|_| Value::List(Vec::new()))
			))
		})
	}
}

fn parser() -> impl Parser<char, Vec<(Value, Value)>, Error = Simple<char>> {
	let value = Value::parser();
	let pair = value.clone().then_ignore(just("\n")).then(value);

	pair.clone()
		.then(just("\n\n").ignore_then(pair).repeated())
		.map(|(first, mut pairs)| {
			pairs.insert(0, first);
			pairs
		})
		.then_ignore(just("\n").repeated())
		.then_ignore(end())
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other) == Ordering::Equal
	}
}

impl Eq for Value {}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Value {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Self::Number(lhs), Self::Number(rhs)) => lhs.cmp(rhs),
			(Self::List(lhs), Self::List(rhs)) => {
				let mut i = 0;
				loop {
					let (lhs, rhs) = (lhs.get(i), rhs.get(i));
					match (lhs, rhs) {
						(None, None) => break Ordering::Equal,
						(None, Some(_)) => break Ordering::Less,
						(Some(_), None) => break Ordering::Greater,
						(Some(lhs), Some(rhs)) => match lhs.cmp(rhs) {
							Ordering::Equal => {},
							ordering => break ordering
						}
					};
					i += 1;
				}
			},

			(Self::Number(lhs), rhs) => Self::List(vec![Self::Number(*lhs)]).cmp(rhs),
			(lhs, Self::Number(rhs)) => lhs.cmp(&Self::List(vec![Self::Number(*rhs)]))
		}
	}
}

fn main() -> anyhow::Result<()> {
	let pairs = read("input.txt", parser())?;

	let mut sum = 0;
	for (i, (lhs, rhs)) in pairs.iter().enumerate() {
		if lhs < rhs {
			sum += i + 1;
		}
	}
	println!("{sum}");

	let mut values = pairs
		.iter()
		.flat_map(|(lhs, rhs)| [lhs, rhs])
		.collect::<BTreeSet<_>>();
	let decoder2 = Value::List(vec![Value::List(vec![Value::Number(2)])]);
	let decoder6 = Value::List(vec![Value::List(vec![Value::Number(6)])]);
	values.insert(&decoder2);
	values.insert(&decoder6);

	let mut key = 1;
	for (i, v) in values.into_iter().enumerate() {
		if v == &decoder2 || v == &decoder6 {
			key *= i + 1;
		}
	}
	println!("{key}");

	Ok(())
}
