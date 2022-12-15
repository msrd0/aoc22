use anyhow::anyhow;
use chumsky::{prelude::*, text::digits};
use std::{
	collections::{BTreeMap, HashSet},
	fs,
	ops::RangeInclusive
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

const LINE_Y: i32 = 2000000;
const MAX: i32 = 4000000;
const MIN: i32 = 0;

#[derive(Debug, Default)]
struct Row {
	ranges: Vec<RangeInclusive<i32>>
}

impl Row {
	fn add_range(&mut self, range: RangeInclusive<i32>) {
		for r in &mut self.ranges {
			// start of the new range is included
			if range.start() >= r.start() && range.start() <= r.end() {
				*r = *r.start() ..= *r.end().max(range.end());
				return;
			}
			// end of the new range is included
			if range.end() >= r.start() && range.end() <= r.end() {
				*r = *r.start().min(range.start()) ..= *r.end();
				return;
			}
			// new range covers the old range
			if range.start() <= r.start() && range.end() >= r.end() {
				*r = range;
				return;
			}
		}
		// range not part of any other range
		self.ranges.push(range);
	}

	fn blocked(&self) -> u32 {
		let mut blocked: u32 = 0;
		let mut last_blocked: Option<i32> = None;

		for r in &self.ranges {
			if let Some(last_blocked) = last_blocked {
				if *r.end() <= last_blocked {
					continue;
				}
				let start = (last_blocked + 1).max(*r.start());
				blocked += (r.end() - start + 1) as u32;
			} else {
				blocked = (r.end() - r.start() + 1) as u32;
			}
			last_blocked = Some(*r.end());
		}

		blocked
	}
}

#[derive(Default)]
struct Map {
	map: BTreeMap<i32, Row>
}

impl Map {
	fn add_x_range(&mut self, range: RangeInclusive<i32>, y: i32) {
		let entry = self.map.entry(y).or_default();
		entry.add_range(range.clone());
		entry.ranges.sort_unstable_by_key(|range| *range.start());
		//eprintln!("{:?} (added {range:?})", entry);
	}

	fn blocked(&self, y: i32) -> u32 {
		self.map
			.get(&y)
			.map(|row| row.blocked())
			.unwrap_or_default()
	}
}

fn main() -> anyhow::Result<()> {
	let sensors = read("input.txt")?;

	let mut map = Map::default();
	let mut beacons_in_line = HashSet::<Position>::new();
	for s in &sensors {
		if s.nearest_beacon.y == LINE_Y {
			beacons_in_line.insert(s.nearest_beacon);
		}

		let beacon_dist = s.pos.manhattan_dist(&s.nearest_beacon);
		let line_dist = s.pos.y.abs_diff(LINE_Y);
		//println!(" - {s:?} (dist: {beacon_dist})");
		if line_dist > beacon_dist {
			continue;
		}
		let xrange = (beacon_dist - line_dist) as i32;
		map.add_x_range(s.pos.x - xrange ..= s.pos.x + xrange, LINE_Y);
	}
	println!("{}", map.blocked(LINE_Y) - beacons_in_line.len() as u32);

	Ok(())
}
