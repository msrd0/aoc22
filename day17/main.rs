use aoc22::read;
use chumsky::prelude::*;
use std::fmt::{self, Debug, Formatter};

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

struct Tetris {
	heights: [usize; 7],
	colors: [Option<u8>; 7],
	max_height: usize
}

impl Tetris {
	fn new() -> Self {
		Self {
			heights: [0; 7],
			colors: [None; 7],
			max_height: 0
		}
	}

	fn is_free(&self, x: usize, y: usize) -> bool {
		self.heights[x] >= y
	}

	fn new_row(&mut self) {
		self.max_height += 1;
		for h in self.heights.iter_mut() {
			*h += 1;
		}
	}

	fn occupy(&mut self, rock: Rock, x: usize, y: usize) {
		if y == 0 {
			panic!("Did you mean to call new_row()?");
		}
		if !self.is_free(x, y) {
			panic!("{x}, {y} is already occupied");
		}
		self.heights[x] = self.heights[x].min(y - 1);
		self.colors[x] = Some(rock as u8);
		debug_assert!(!self.is_free(x, y));
	}
}

impl Debug for Tetris {
	#[allow(clippy::comparison_chain)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "│")?;
		for x in 0 .. 7 {
			write!(f, "{:2}", self.heights[x])?;
		}
		writeln!(f, "│")?;
		for y in 0 .. self.max_height {
			let mut last = true;
			write!(f, "│")?;
			for x in 0 .. 7 {
				if y < self.heights[x] {
					write!(f, "  ")?;
					last = false;
				} else if y == self.heights[x] {
					write!(f, "\x1B[{}m██\x1B[0m", 31 + self.colors[x].unwrap())?;
					last = false;
				} else {
					write!(f, "??")?;
				}
			}
			writeln!(f, "│")?;
			if last {
				break;
			}
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
				let lines = self.lines();
				let size = self.size();
				for i in 0 .. size.height {
					if y >= i {
						for j in 0 .. size.width {
							if !tetris.is_free(x+j, y-i) && lines[i][j] != b' ' {
								return true;
							}
						}
					}
				}
				false
			}

			fn freeze(self, tetris: &mut Tetris, x: usize, y: usize) {
				let lines = self.lines();
				let size = self.size();
				for i in 0..size.height {
					if y > i {
						for j in 0..size.width {
							if lines[i][j] != b' ' {
								tetris.occupy(self, x+j, y-i);
							}
						}
					} else {
						tetris.new_row();
						for j in 0..size.width {
							if lines[i][j] != b' ' {
								tetris.occupy(self, x+j, 1);
							}
						}
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
	let Size { width, .. } = rock.size();
	let dir = wind.next().unwrap();
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
		let debug = i == 58 || i == 59;

		let rock = Rock::from_index(i);
		if rock == Rock::HorizLine && wind.1 == 0 {
			println!("Reset at i={i}");
			if stop_at_reset {
				return Some(i);
			}
		}

		let mut x: usize = 2;
		let mut y: usize = 0;
		if debug {
			println!("{rock:?} starts falling at ({x}, -)");
		}
		for _ in 0 .. 4 {
			apply_wind(wind, rock, &mut x);
			if debug {
				println!("{rock:?} was pushed to     ({x}, -)");
			}
		}
		while !rock.intersects(tetris, x, y + 1) {
			y += 1;
			if debug {
				println!("{rock:?} falls down to     ({x}, {y})");
			}
			let backup = x;
			apply_wind(wind, rock, &mut x);
			if rock.intersects(tetris, x, y) {
				x = backup;
			} else if debug {
				println!("{rock:?} was pushed to     ({x}, {y})");
			}
		}

		if debug {
			println!("{rock:?} freezes at        ({x}, {y})");
		}
		rock.freeze(tetris, x, y);

		if debug {
			println!("i={i} (rock: {rock:?}):");
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
	println!("{}", tetris.max_height);

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
	let lcm_height = tetris.max_height;
	simulate(&mut wind, &mut tetris, lcm .. lcm * 2, false);
	let lcm2_height = tetris.max_height;
	let multiplier = 1000000000000 / lcm;
	let rem = 1000000000000 % lcm;
	simulate(&mut wind, &mut tetris, lcm .. lcm + rem, false);
	dbg!(lcm);
	dbg!(lcm_height);
	dbg!(lcm2_height);
	dbg!(multiplier);
	dbg!(rem);
	dbg!(tetris.max_height);
	println!(
		"{}",
		(lcm2_height - lcm_height) * (multiplier - 1) + tetris.max_height
	);

	Ok(())
}
