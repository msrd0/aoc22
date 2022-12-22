use aoc22::read;
use chumsky::{prelude::*, text::digits};
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum Facing {
	Right = 0,
	Down = 1,
	Left = 2,
	Up = 3
}

impl AddAssign<usize> for Facing {
	fn add_assign(&mut self, rhs: usize) {
		*self = match (*self as usize + rhs) % 4 {
			0 => Self::Right,
			1 => Self::Down,
			2 => Self::Left,
			3 => Self::Up,
			_ => unreachable!()
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
	Void,
	Open,
	Wall
}

impl Tile {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		choice((
			just(" ").map(|_| Self::Void),
			just(".").map(|_| Self::Open),
			just("#").map(|_| Self::Wall)
		))
	}
}

struct Map {
	rows: Vec<Vec<Tile>>
}

impl Map {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
		Tile::parser()
			.repeated()
			.at_least(1)
			.then_ignore(just("\n"))
			.repeated()
			.at_least(1)
			.map(|rows| Self { rows })
	}
}

#[derive(Debug)]
enum Instruction {
	Move(usize),
	TurnClockwise,
	TurnAnticlockwise
}

impl Instruction {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		choice((
			digits(10).map(|digits: String| Self::Move(digits.parse().unwrap())),
			just("R").map(|_| Self::TurnClockwise),
			just("L").map(|_| Self::TurnAnticlockwise)
		))
	}
}

fn parser() -> impl Parser<char, (Map, Vec<Instruction>), Error = Simple<char>> {
	Map::parser()
		.then_ignore(just("\n"))
		.then(Instruction::parser().repeated().at_least(1))
		.then_ignore(just("\n"))
		.then_ignore(end())
}

fn main() -> anyhow::Result<()> {
	let (map, instructions) = read("input.txt", parser())?;

	let mut y = 0;
	let mut x = map.rows[y]
		.iter()
		.position(|tile| *tile == Tile::Open)
		.unwrap();
	let mut facing = Facing::Right;
	for inst in instructions {
		match (inst, facing) {
			(Instruction::TurnClockwise, _) => facing += 1,
			(Instruction::TurnAnticlockwise, _) => facing += 3,

			(Instruction::Move(steps), Facing::Right) => {
				let row = &map.rows[y];
				for _ in 0 .. steps {
					let mut next = x + 1;
					if next >= row.len() {
						next = row.iter().position(|tile| *tile != Tile::Void).unwrap();
					}
					if row[next] == Tile::Wall {
						break;
					}
					x = next;
				}
			},

			(Instruction::Move(steps), Facing::Left) => {
				let row = &map.rows[y];
				let first = row.iter().position(|tile| *tile != Tile::Void).unwrap();
				for _ in 0 .. steps {
					let next = if x == 0 || x - 1 < first {
						row.len() - 1
					} else {
						x - 1
					};
					if row[next] == Tile::Wall {
						break;
					}
					x = next;
				}
			},

			(Instruction::Move(steps), Facing::Up) => {
				let mut i = 0;
				let mut next = y;
				while i < steps {
					if next == 0 {
						next = map.rows.len() - 1;
					} else {
						next -= 1;
					}
					match map.rows[next].get(x).copied().unwrap_or(Tile::Void) {
						Tile::Open => {
							y = next;
							i += 1;
						},
						Tile::Wall => {
							break;
						},
						Tile::Void => {}
					}
				}
			},

			(Instruction::Move(steps), Facing::Down) => {
				let mut i = 0;
				let mut next = y;
				while i < steps {
					next += 1;
					if next >= map.rows.len() {
						next = 0;
					}
					match map.rows[next].get(x).copied().unwrap_or(Tile::Void) {
						Tile::Open => {
							y = next;
							i += 1;
						},
						Tile::Wall => {
							break;
						},
						Tile::Void => {}
					}
				}
			}
		}
	}
	println!("x={x}, y={y}, facing={facing:?}");
	println!("{}", 1000 * (y + 1) + 4 * (x + 1) + facing as usize);

	Ok(())
}
