use aoc22::read;
use chumsky::{prelude::*, text::digits};
use paste::paste;
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

fn move_plain(map: &Map, x: &mut usize, y: &mut usize, facing: &mut Facing) -> bool {
	match *facing {
		Facing::Right => {
			let row = &map.rows[*y];
			let mut next = *x + 1;
			if next >= row.len() {
				next = row.iter().position(|tile| *tile != Tile::Void).unwrap();
			}
			if row[next] == Tile::Wall {
				return false;
			}
			*x = next;
		},

		Facing::Left => {
			let row = &map.rows[*y];
			let first = row.iter().position(|tile| *tile != Tile::Void).unwrap();
			let next = if *x == 0 || *x - 1 < first {
				row.len() - 1
			} else {
				*x - 1
			};
			if row[next] == Tile::Wall {
				return false;
			}
			*x = next;
		},

		Facing::Down => {
			let mut next = *y;
			loop {
				next += 1;
				if next >= map.rows.len() {
					next = 0;
				}
				match map.rows[next].get(*x).copied().unwrap_or(Tile::Void) {
					Tile::Open => break,
					Tile::Wall => return false,
					Tile::Void => {}
				}
			}
			*y = next;
		},

		Facing::Up => {
			let mut next = *y;
			loop {
				if next == 0 {
					next = map.rows.len() - 1;
				} else {
					next -= 1;
				}
				match map.rows[next].get(*x).copied().unwrap_or(Tile::Void) {
					Tile::Open => break,
					Tile::Wall => return false,
					Tile::Void => {}
				}
			}
			*y = next;
		}
	}
	true
}

macro_rules! regions {
	($($region:literal: ($x:literal, $y:literal)),*) => {
		paste! {
			$(
				const [<REGION_ $region _X>]: usize = $x;
				const [<REGION_ $region _X_END>]: usize = $x + 49;
				const [<REGION_ $region _Y>]: usize = $y;
				const [<REGION_ $region _Y_END>]: usize = $y + 49;
			)*

			#[derive(Clone, Copy, Debug, Eq, PartialEq)]
			enum Region {
				$([<Region $region>] = $region),*
			}

			impl Region {
				fn x(self) -> usize {
					match (self) {
						$(Region::[<Region $region>] => $x),*
					}
				}

				fn y(self) -> usize {
					match (self) {
						$(Region::[<Region $region>] => $y),*
					}
				}
			}

			// return the region for a certain coordinate
			fn region(x: usize, y: usize) -> Region {
				match (x, y) {
					$((
						[<REGION_ $region _X>] ..= [<REGION_ $region _X_END>],
						[<REGION_ $region _Y>] ..= [<REGION_ $region _Y_END>]
					) => Region::[<Region $region>],)*
					_ => unreachable!()
				}
			}
		}
	};
}

// layout:
//   1122
//   44
// 5566
// 33
regions! {
	1: (50, 0),
	2: (100, 0),
	4: (50, 50),
	5: (0, 100),
	6: (50, 100),
	3: (0, 150)
}

fn move_on_cube(map: &Map, x: &mut usize, y: &mut usize, facing: &mut Facing) -> bool {
	// save start coordinates
	let sx = *x;
	let sy = *y;
	let sf = *facing;
	let sr = region(sx, sy);

	// move on plain. if region doesn't change, we're good
	let pres = move_plain(map, x, y, facing);
	debug_assert_eq!(*facing, sf);
	let pr = region(*x, *y);
	if sr == pr {
		return pres;
	}

	// otherwise, we might need to wrap on cube
	let (tr, tf) = match (sr, sf) {
		(Region::Region1, Facing::Left) => (Region::Region5, Facing::Right),
		(Region::Region1, Facing::Up) => (Region::Region3, Facing::Right),

		(Region::Region2, Facing::Right) => (Region::Region6, Facing::Left),
		(Region::Region2, Facing::Down) => (Region::Region4, Facing::Left),
		(Region::Region2, Facing::Up) => (Region::Region3, Facing::Up),

		(Region::Region3, Facing::Right) => (Region::Region6, Facing::Up),
		(Region::Region3, Facing::Left) => (Region::Region1, Facing::Down),
		(Region::Region3, Facing::Down) => (Region::Region2, Facing::Down),

		(Region::Region4, Facing::Right) => (Region::Region2, Facing::Up),
		(Region::Region4, Facing::Left) => (Region::Region5, Facing::Down),

		(Region::Region5, Facing::Left) => (Region::Region1, Facing::Right),
		(Region::Region5, Facing::Up) => (Region::Region4, Facing::Right),

		(Region::Region6, Facing::Right) => (Region::Region2, Facing::Left),
		(Region::Region6, Facing::Down) => (Region::Region3, Facing::Left),

		// the rest works exactly the same as in the plane
		_ => return pres
	};

	// perform the wrapping on the cube
	match (sf, tf) {
		(sf, tf) if sf == tf => {
			*x = sx - sr.x() + tr.x();
			*y = sy - sr.y() + tr.y();
		},

		(Facing::Left, Facing::Right) | (Facing::Right, Facing::Left) => {
			*x = sx - sr.x() + tr.x(); // flip right/left
			*y = 49 - (sy - sr.y()) + tr.y(); // actual position
		},

		(Facing::Up, Facing::Right)
		| (Facing::Right, Facing::Up)
		| (Facing::Down, Facing::Left)
		| (Facing::Left, Facing::Down) => {
			*x = sy - sr.y() + tr.x();
			*y = sx - sr.x() + tr.y();
		},

		(sf, tf) => unimplemented!("sf={sf:?}, tf={tf:?}")
	};
	*facing = tf;
	debug_assert_eq!(tr, region(*x, *y));

	// if we hit a wall, undo everything
	if map.rows[*y][*x] == Tile::Wall {
		*x = sx;
		*y = sy;
		*facing = sf;
		return false;
	}
	true
}

fn run<F>(map: &Map, instructions: &[Instruction], move_callback: F) -> usize
where
	F: Fn(&Map, &mut usize, &mut usize, &mut Facing) -> bool
{
	let mut y = 0;
	let mut x = map.rows[y]
		.iter()
		.position(|tile| *tile == Tile::Open)
		.unwrap();
	let mut facing = Facing::Right;

	for inst in instructions {
		match inst {
			Instruction::TurnClockwise => facing += 1,
			Instruction::TurnAnticlockwise => facing += 3,
			Instruction::Move(steps) => {
				for _ in 0 .. *steps {
					if !move_callback(map, &mut x, &mut y, &mut facing) {
						break;
					}
				}
			},
		}
	}

	1000 * (y + 1) + 4 * (x + 1) + facing as usize
}

fn main() -> anyhow::Result<()> {
	let (map, instructions) = read("input.txt", parser())?;

	// part 1
	println!("{}", run(&map, &instructions, move_plain));
	// part 2
	// 120345 is too high
	println!("{}", run(&map, &instructions, move_on_cube));

	Ok(())
}
