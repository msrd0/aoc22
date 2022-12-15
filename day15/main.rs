use anyhow::anyhow;
use chumsky::{prelude::*, text::digits};
use std::{collections::BTreeMap, fs};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
	x: i32,
	y: i32
}

impl Position {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		let num = choice((
			digits(10).map(|digits: String| digits.parse().unwrap()),
			just("-")
				.ignore_then(digits(10))
				.map(|digits: String| -(digits.parse::<i32>().unwrap()))
		));
		just("x=")
			.ignore_then(num)
			.then_ignore(just(", y="))
			.then(num)
			.map(|(x, y)| Self { x, y })
	}

	fn manhattan_dist(&self, other: &Self) -> u32 {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}
}

#[derive(Clone, Copy, Debug)]
struct Sensor {
	pos: Position,
	nearest_beacon: Position
}

impl Sensor {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		just("Sensor at ")
			.ignore_then(Position::parser())
			.then_ignore(just(": closest beacon is at "))
			.then(Position::parser())
			.map(|(pos, nearest_beacon)| Self {
				pos,
				nearest_beacon
			})
	}
}

fn parser() -> impl Parser<char, Vec<Sensor>, Error = Simple<char>> {
	Sensor::parser()
		.then_ignore(just("\n"))
		.repeated()
		.at_least(1)
		.then_ignore(end())
}

fn read<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Vec<Sensor>> {
	parser().parse(fs::read_to_string(path)?).map_err(|errors| {
		anyhow!(errors
			.into_iter()
			.map(|err| err.to_string())
			.collect::<Vec<_>>()
			.join("\n"))
	})
}

fn main() -> anyhow::Result<()> {
	let sensors = read("input.txt")?;

	// true if a sensor can reach
	let mut line = BTreeMap::<i32, bool>::new();
	let line_y = 2000000;

	for s in &sensors {
		if s.nearest_beacon.y == line_y {
			line.insert(s.nearest_beacon.x, false);
		}

		let beacon_dist = s.pos.manhattan_dist(&s.nearest_beacon);
		let line_dist = s.pos.y.abs_diff(line_y);
		if line_dist > beacon_dist {
			continue;
		}
		let xrange = (beacon_dist - line_dist) as i32;
		for x in s.pos.x - xrange ..= s.pos.x + xrange {
			line.entry(x).or_insert(true);
		}
	}
	println!("{}", line.values().filter(|it| **it).count());

	Ok(())
}
