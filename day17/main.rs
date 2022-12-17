use aoc22::read;
use chumsky::prelude::*;
use std::{
	collections::VecDeque,
	fmt::{self, Debug, Display, Formatter},
	ops::Range
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
	Left,
	Right
}

struct Wind(Vec<Direction>, usize);

impl Iterator for Wind {
	type Item = Direction;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.0[self.1];
		self.1 = (self.1 + 1) % self.0.len();
		Some(next)
	}
}

fn parser() -> impl Parser<char, Wind, Error = Simple<char>> {
	choice((
		just(">").map(|_| Direction::Right),
		just("<").map(|_| Direction::Left)
	))
	.repeated()
	.at_least(1)
	.then_ignore(just("\n"))
	.then_ignore(end())
	.map(|wind| Wind(wind, 0))
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
	#[default]
	Free,
	Rock(u8),
	Bottom
}

impl Display for Tile {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Free => f.write_str("  "),
			Self::Rock(color) => write!(f, "\x1B[{}m██\x1B[0m", 31 + color),
			Self::Bottom => f.write_str("_")
		}
	}
}

struct Tetris {
	rows: VecDeque<[Tile; 7]>
}

impl Tetris {
	fn new() -> Self {
		Self {
			rows: VecDeque::new()
		}
	}

	fn row(&self, y: usize) -> &[Tile; 7] {
		if y == 0 {
			return &[Tile::Free; 7];
		}
		let y = y - 1;
		if y >= self.rows.len() {
			return &[Tile::Bottom; 7];
		}
		&self.rows[y]
	}

	fn row_mut(&mut self, y: usize) -> &mut [Tile; 7] {
		if y < 1 || y > self.rows.len() {
			panic!("Index {y} out of bounds");
		}
		&mut self.rows[y - 1]
	}
}

impl Debug for Tetris {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		for row in &self.rows {
			writeln!(
				f,
				"│{}{}{}{}{}{}{}│",
				row[0], row[1], row[2], row[3], row[4], row[5], row[6]
			)?;
		}
		f.write_str("┕━━━━━━━━━━━━━━┙")
	}
}

struct Size {
	width: usize,
	height: usize
}

macro_rules! rocks {
	($($ident:ident : { $($line:literal),+ }),+) => {
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		#[repr(u8)]
		enum Rock {
			$($ident),+
		}

		impl Rock {
			fn from_index(idx: usize) -> Self {
				let mut len = 0;
				$(stringify!($ident); len += 1;)+
				let idx = idx % len;
				let mut curr = 0;
				$(
					if curr == idx {
						return Self::$ident;
					}
					curr += 1;
				)+
				unreachable!("index {idx} cannot be reached when len is {len} (curr is {curr})")
			}

			const fn lines(self) -> &'static [&'static [u8]] {
				match self {
					$(Self::$ident => &[$($line),+]),+
				}
			}

			#[allow(unused_must_use)]
			fn size(self) -> Size {
				match self {
					$(Self::$ident => Size {
						width: { $($line.len());+ },
						height: [$($line),+].len()
					}),+
				}
			}

			fn intersects(self, tetris: &Tetris, x: usize, y: usize) -> bool {
				// println!("intersects({self:?}, tetris, x={x}, y={y})");
				let lines = self.lines();
				let size = self.size();
				for i in 0 .. size.height {
					if y >= i {
						// println!("  -> checking i={i}");
						for j in 0 .. size.width {
							// println!("      -> tetris.row(y-i)[x+j] = {:?}", tetris.row(y-i)[x+j]);
							// println!("         lines[i][j] = '{}'", lines[i][j] as char);
							if tetris.row(y-i)[x+j] != Tile::Free && lines[i][j] != b' ' {
								return true;
							}
						}
					}
				}
				false
			}

			fn freeze(self, tetris: &mut Tetris, x: usize, y: usize) {
				// println!("freeze({self:?}, tetris, x={x}, y={y}");
				let lines = self.lines();
				let size = self.size();
				for i in 0..size.height {
					if y > i {
						// println!("  -> Updating row {y}-{i}");
						for j in 0..size.width {
							if lines[i][j] != b' ' {
								tetris.row_mut(y-i)[x+j] = Tile::Rock(self as u8);
							}
						}
					} else {
						// println!("  -> Inserting new row");
						let mut row = [Tile::Free; 7];
						for j in 0..size.width {
							if lines[i][j] != b' ' {
								row[x+j] = Tile::Rock(self as u8);
							}
						}
						tetris.rows.push_front(row);
					}
				}
			}
		}
	};
}

rocks! {
	HorizLine: {
		b"####"
	},
	Plus: {
		b" # ",
		b"###",
		b" # "
	},
	Corner: {
		b"###",
		b"  #",
		b"  #"
	},
	VertLine: {
		b"#",
		b"#",
		b"#",
		b"#"
	},
	Square: {
		b"##",
		b"##"
	}
}

fn apply_wind(wind: &mut Wind, rock: Rock, x: &mut usize) {
	// println!("apply_wind(wind, rock={rock:?}, x={x})");
	let Size { width, .. } = rock.size();
	let dir = wind.next().unwrap();
	// println!("  -> wind is pushing {dir:?}");
	match dir {
		Direction::Left => *x = x.saturating_sub(1),
		Direction::Right => {
			if *x + 1 + width <= 7 {
				*x += 1;
			}
		},
	}
}

fn simulate<I>(
	wind: &mut Wind,
	tetris: &mut Tetris,
	range: I,
	stop_at_reset: bool
) -> Option<usize>
where
	I: Iterator<Item = usize>
{
	for i in range {
		let rock = Rock::from_index(i);
		if rock == Rock::HorizLine && wind.1 == 0 {
			println!("Reset at i={i}");
			if stop_at_reset {
				return Some(i);
			}
		}

		let mut x: usize = 2;
		let mut y: usize = 0;
		for _ in 0 .. 4 {
			apply_wind(wind, rock, &mut x);
		}
		while !rock.intersects(tetris, x, y + 1) {
			y += 1;
			let backup = x;
			apply_wind(wind, rock, &mut x);
			if rock.intersects(tetris, x, y) {
				x = backup;
			}
		}

		rock.freeze(tetris, x, y);

		if i == 81 {
			println!("{tetris:?}");
		}
	}
	None
}

fn main() -> anyhow::Result<()> {
	let mut wind = read("input.txt", parser())?;

	let mut idx = 2022;
	let mut tetris = Tetris::new();
	simulate(&mut wind, &mut tetris, 0 .. idx, false);
	println!("{}", tetris.rows.len());

	// part 2
	idx = simulate(&mut wind, &mut tetris, idx .., true).unwrap();
	dbg!(idx);

	let lcm = wind.0.len() * 5 * 3;
	if lcm < 2022 {
		tetris = Tetris::new();
		wind.1 = 0;
		idx = 0;
	}
	simulate(&mut wind, &mut tetris, idx .. lcm, false);
	let lcm_height = tetris.rows.len();
	simulate(&mut wind, &mut tetris, lcm .. lcm * 2, false);
	let lcm2_height = tetris.rows.len();
	let multiplier = 1000000000000 / lcm;
	let rem = 1000000000000 % lcm;
	simulate(&mut wind, &mut tetris, lcm .. lcm + rem, false);
	dbg!(lcm);
	dbg!(lcm_height);
	dbg!(lcm2_height);
	dbg!(multiplier);
	dbg!(rem);
	dbg!(tetris.rows.len());
	println!(
		"{}",
		(lcm2_height - lcm_height) * (multiplier - 1) + tetris.rows.len()
	);

	Ok(())
}
