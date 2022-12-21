use aoc22::read;
use chumsky::{
	prelude::*,
	text::{digits, ident}
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Op {
	Add,
	Sub,
	Mul,
	Div
}

impl Op {
	fn apply(self, lhs: i64, rhs: i64) -> i64 {
		match self {
			Self::Add => lhs + rhs,
			Self::Sub => lhs - rhs,
			Self::Mul => lhs * rhs,
			Self::Div => lhs / rhs
		}
	}

	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		choice((
			just("+").map(|_| Self::Add),
			just("-").map(|_| Self::Sub),
			just("*").map(|_| Self::Mul),
			just("/").map(|_| Self::Div)
		))
	}
}

enum Expr {
	Literal(i64),
	Op(String, Op, String)
}

impl Expr {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		choice((
			digits(10).map(|digits: String| Self::Literal(digits.parse().unwrap())),
			ident()
				.then_ignore(just(" "))
				.then(Op::parser())
				.then_ignore(just(" "))
				.then(ident())
				.map(|((lhs, op), rhs)| Self::Op(lhs, op, rhs))
		))
	}
}

fn parser() -> impl Parser<char, HashMap<String, Expr>, Error = Simple<char>> {
	ident()
		.then_ignore(just(": "))
		.then(Expr::parser())
		.then_ignore(just("\n"))
		.repeated()
		.at_least(1)
		.collect()
		.then_ignore(end())
}

fn evaluate(name: &str, exprs: &mut HashMap<String, Expr>) -> i64 {
	let (lhs, op, rhs) = match &exprs[name] {
		Expr::Literal(lit) => return *lit,
		Expr::Op(lhs, op, rhs) => (lhs.clone(), *op, rhs.clone())
	};
	let lhs = evaluate(&lhs, exprs);
	let rhs = evaluate(&rhs, exprs);
	let value = op.apply(lhs, rhs);
	*exprs.get_mut(name).unwrap() = Expr::Literal(value);
	value
}

fn main() -> anyhow::Result<()> {
	let mut exprs = read("input.txt", parser())?;
	let root = evaluate("root", &mut exprs);
	println!("{root}");
	Ok(())
}
