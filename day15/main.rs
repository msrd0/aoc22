use aoc22::read;
use chumsky::{prelude::*, text::digits};
use std::{
	collections::{BTreeMap, HashSet},
	ops::RangeInclusive
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
	x: i64,
	y: i64
}

impl Position {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		let num = choice((
			digits(10).map(|digits: String| digits.parse().unwrap()),
			just("-")
				.ignore_then(digits(10))
				.map(|digits: String| -(digits.parse::<i64>().unwrap()))
		));
		just("x=")
			.ignore_then(num)
			.then_ignore(just(", y="))
			.then(num)
			.map(|(x, y)| Self { x, y })
	}

	fn manhattan_dist(&self, other: &Self) -> u64 {
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

// const LINE_Y: i64 = 10;
// const MAX: i64 = 20;
// const MIN: i64 = 0;

const LINE_Y: i64 = 2000000;
const MAX: i64 = 4000000;
const MIN: i64 = 0;

#[derive(Debug, Default)]
struct Row {
	ranges: Vec<RangeInclusive<i64>>
}

impl Row {
	fn add_range(&mut self, range: RangeInclusive<i64>) {
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

	fn normalize(&mut self) {
		self.ranges.sort_unstable_by_key(|range| *range.start());
		let mut i = 0;
		while i < self.ranges.len() - 1 {
			if self.ranges[i].end() + 1 >= *self.ranges[i + 1].start() {
				self.ranges[i] = *self.ranges[i].start()
					..= *self.ranges[i].end().max(self.ranges[i + 1].end());
				self.ranges.remove(i + 1);
				continue;
			}
			i += 1;
		}
	}

	fn blocked(&self) -> u64 {
		let mut blocked: u64 = 0;
		let mut last_blocked: Option<i64> = None;

		for r in &self.ranges {
			if let Some(last_blocked) = last_blocked {
				if *r.end() <= last_blocked {
					continue;
				}
				let start = (last_blocked + 1).max(*r.start());
				blocked += (r.end() - start + 1) as u64;
			} else {
				blocked = (r.end() - r.start() + 1) as u64;
			}
			last_blocked = Some(*r.end());
		}

		blocked
	}

	fn find_free(&self) -> Vec<i64> {
		let mut last = MIN - 1;
		let mut free = Vec::new();
		for r in &self.ranges {
			if *r.start() > last + 1 {
				free.push(last + 1);
			}
			last = *r.end();
		}
		free
	}
}

#[derive(Default)]
struct Map {
	map: BTreeMap<i64, Row>
}

impl Map {
	fn add_x_range(&mut self, range: RangeInclusive<i64>, y: i64) {
		if range.start() > range.end() {
			panic!("range={range:?}");
		}

		let entry = self.map.entry(y).or_default();
		entry.add_range(range);
		entry.normalize();
	}

	fn blocked(&self, y: i64) -> u64 {
		self.map
			.get(&y)
			.map(|row| row.blocked())
			.unwrap_or_default()
	}

	fn find_free(&self) -> Vec<Position> {
		let mut positions = vec![];
		for y in MIN ..= MAX {
			if y % 100000 == 0 {
				println!("find_free(): y={y}");
			}

			if let Some(row) = self.map.get(&y) {
				for x in row.find_free() {
					positions.push(Position { x, y });
				}
			} else {
				eprintln!("Missing row for y={y}");
			}
		}
		positions
	}
}

fn main() -> anyhow::Result<()> {
	let sensors = read("input.txt", parser())?;

	let mut map = Map::default();
	let mut beacons_in_line = HashSet::<Position>::new();
	for s in &sensors {
		if s.nearest_beacon.y == LINE_Y {
			beacons_in_line.insert(s.nearest_beacon);
		}

		let beacon_dist = s.pos.manhattan_dist(&s.nearest_beacon);
		let line_dist = s.pos.y.abs_diff(LINE_Y);
		if line_dist > beacon_dist {
			continue;
		}
		let xrange = (beacon_dist - line_dist) as i64;
		map.add_x_range(s.pos.x - xrange ..= s.pos.x + xrange, LINE_Y);
	}
	println!("{}", map.blocked(LINE_Y) - beacons_in_line.len() as u64);

	// part 2

	map = Map::default();
	for s in &sensors {
		println!(" - {s:?}");
		let beacon_dist = s.pos.manhattan_dist(&s.nearest_beacon) as i64;
		for y in (s.pos.y - beacon_dist).max(MIN) ..= (s.pos.y + beacon_dist).min(MAX) {
			let xrange = beacon_dist - s.pos.y.abs_diff(y) as i64;
			map.add_x_range(
				(s.pos.x - xrange).max(MIN) ..= (s.pos.x + xrange).min(MAX),
				y
			);
		}
	}
	for free in map.find_free() {
		println!("{free:?} = {}", free.x * 4000000 + free.y);
	}

	Ok(())
}
