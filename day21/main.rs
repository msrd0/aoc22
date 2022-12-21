use anyhow::bail;
use aoc22::read;
use chumsky::{
	prelude::*,
	text::{digits, ident}
};
use std::{
	collections::HashMap,
	fmt::{self, Display, Formatter}
};

#[derive(Clone, Copy)]
enum Op {
	Add,
	Sub,
	Mul,
	Div
}

impl Display for Op {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Add => "+",
			Self::Sub => "-",
			Self::Mul => "*",
			Self::Div => "/"
		})
	}
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

	fn is_commutative(self) -> bool {
		matches!(self, Self::Add | Self::Mul)
	}

	fn invert(self) -> Self {
		match self {
			Self::Add => Self::Sub,
			Self::Sub => Self::Add,
			Self::Mul => Self::Div,
			Self::Div => Self::Mul
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

#[derive(Clone)]
enum Expr {
	Literal(i64),
	Variable(String),
	Op(Box<Expr>, Op, Box<Expr>)
}

impl Display for Expr {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Literal(lit) => write!(f, "{lit}"),
			Self::Variable(var) => write!(f, "{var}"),
			Self::Op(lhs, op, rhs) => write!(f, "({lhs} {op} {rhs})")
		}
	}
}

impl Expr {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		let var = ident().map(|ident| Box::new(Self::Variable(ident)));
		choice((
			digits(10).map(|digits: String| Self::Literal(digits.parse().unwrap())),
			var.then_ignore(just(" "))
				.then(Op::parser())
				.then_ignore(just(" "))
				.then(var)
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

fn evaluate_side(expr: Expr, exprs: &mut HashMap<String, Expr>) -> Expr {
	match expr {
		// this was either a literal to begin with or calculated before
		Expr::Literal(lit) => Expr::Literal(lit),
		// this was either never calculated or determined to be unavailable
		Expr::Variable(var) => evaluate(&var, exprs)
			.map(Expr::Literal)
			.or_else(|| exprs.get(&var).cloned())
			.unwrap_or(Expr::Variable(var)),
		// this is an expression that cannot be calculated right now
		expr => expr
	}
}

fn evaluate(name: &str, exprs: &mut HashMap<String, Expr>) -> Option<i64> {
	let (lhs, op, rhs) = match exprs.get(name)?.clone() {
		Expr::Literal(lit) => return Some(lit),
		Expr::Variable(var) => return evaluate(&var, exprs),
		Expr::Op(lhs, op, rhs) => (*lhs, op, *rhs)
	};
	let lhs = evaluate_side(lhs, exprs);
	let rhs = evaluate_side(rhs, exprs);
	match (lhs, rhs) {
		(Expr::Literal(lhs), Expr::Literal(rhs)) => {
			let value = op.apply(lhs, rhs);
			*exprs.get_mut(name).unwrap() = Expr::Literal(value);
			Some(value)
		},
		(Expr::Variable(_), Expr::Variable(_)) => None,
		(lhs, rhs) => {
			*exprs.get_mut(name).unwrap() = Expr::Op(Box::new(lhs), op, Box::new(rhs));
			None
		}
	}
}

fn main() -> anyhow::Result<()> {
	let input = read("input.txt", parser())?;

	let mut exprs = input.clone();
	let root = evaluate("root", &mut exprs).unwrap();
	println!("{root}");

	// part 2

	let mut exprs = input;
	exprs.remove("humn");
	evaluate("root", &mut exprs);
	let Expr::Op(lhs, _, rhs) = exprs["root"].clone() else { bail!("weird root") };
	let (mut value, mut expr) = match (*lhs, *rhs) {
		(Expr::Literal(value), expr) => (value, expr),
		(expr, Expr::Literal(value)) => (value, expr),
		_ => bail!("found no value")
	};
	loop {
		match expr {
			Expr::Literal(_) => bail!("how?!?"),
			Expr::Variable(var) => {
				assert_eq!(var, "humn");
				println!("{value}");
				break;
			},
			Expr::Op(lhs, op, rhs) => match (*lhs, *rhs) {
				(lhs, Expr::Literal(rhs)) => {
					value = op.invert().apply(value, rhs);
					expr = lhs;
				},
				(Expr::Literal(lhs), rhs) if op.is_commutative() => {
					value = op.invert().apply(value, lhs);
					expr = rhs;
				},
				(Expr::Literal(lhs), rhs) => {
					expr = Expr::Op(
						Box::new(Expr::Literal(value)),
						op.invert(),
						Box::new(rhs)
					);
					value = lhs;
				},
				(lhs, rhs) => {
					println!("{value} = {lhs} {op} {rhs}");
					break;
				}
			}
		}
	}

	Ok(())
}
