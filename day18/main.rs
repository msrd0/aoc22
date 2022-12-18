use aoc22::read;
use chumsky::{prelude::*, text::digits};
use indexmap::IndexSet;
use std::{
	collections::{HashSet, VecDeque},
	ops::{Add, Sub}
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position<T = i32> {
	x: T,
	y: T,
	z: T
}

impl<T: Add> Add for Position<T> {
	type Output = Position<T::Output>;

	fn add(self, rhs: Self) -> Self::Output {
		Position {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

impl<T: Sub> Sub for Position<T> {
	type Output = Position<T::Output>;

	fn sub(self, rhs: Self) -> Self::Output {
		Position {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z
		}
	}
}

impl<T> Position<T> {
	fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
}

impl Position<i32> {
	fn adjacent(self) -> [Self; 6] {
		[
			self - Self::new(1, 0, 0),
			self + Self::new(1, 0, 0),
			self - Self::new(0, 1, 0),
			self + Self::new(0, 1, 0),
			self - Self::new(0, 0, 1),
			self + Self::new(0, 0, 1)
		]
	}
}

fn parser() -> impl Parser<char, IndexSet<Position>, Error = Simple<char>> {
	let pos = digits(10)
		.then_ignore(just(","))
		.then(digits(10))
		.then_ignore(just(","))
		.then(digits(10))
		.map(|((x, y), z): ((String, String), String)| Position {
			x: x.parse().unwrap(),
			y: y.parse().unwrap(),
			z: z.parse().unwrap()
		});
	pos.then_ignore(just("\n"))
		.repeated()
		.at_least(1)
		.map(|vec| vec.into_iter().collect())
}

fn main() -> anyhow::Result<()> {
	let cubes = read("input.txt", parser())?;

	let mut count: u64 = 0;
	for c in &cubes {
		for adj in c.adjacent() {
			if !cubes.contains(&adj) {
				count += 1;
			}
		}
	}
	println!("{count}");

	// part 2

	let (max_x, max_y, max_z) = cubes.iter().fold((1, 1, 1), |(x, y, z), cube| {
		(x.max(cube.x + 1), y.max(cube.y + 1), z.max(cube.z + 1))
	});
	let mut count: u64 = 0;
	let mut visited = HashSet::new();
	let mut q = VecDeque::new();
	q.push_back(Position::new(-1, -1, -1));
	while let Some(steam) = q.pop_front() {
		if steam.x > max_x
			|| steam.y > max_y
			|| steam.z > max_z
			|| steam.x < -1
			|| steam.y < -1
			|| steam.z < -1
			|| visited.contains(&steam)
		{
			continue;
		}
		visited.insert(steam);

		for adj in steam.adjacent() {
			if cubes.contains(&adj) {
				count += 1;
			} else {
				q.push_back(adj);
			}
		}
	}
	println!("{count}");

	Ok(())
}
