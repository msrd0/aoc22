use anyhow::anyhow;
use std::{
	collections::HashSet,
	fs::File,
	io::{BufRead as _, BufReader},
	str::FromStr
};

const DEBUG: bool = false;

#[derive(Debug)]
struct Move {
	direction: char,
	length: i32
}

impl FromStr for Move {
	type Err = anyhow::Error;

	fn from_str(line: &str) -> anyhow::Result<Self> {
		let mut split = line.split(' ');
		let direction = split
			.next()
			.ok_or_else(|| anyhow!("Line too short"))?
			.chars()
			.next()
			.ok_or_else(|| anyhow!("Huh?"))?;
		let length = split
			.next()
			.ok_or_else(|| anyhow!("Line too short"))?
			.parse()?;
		Ok(Self { direction, length })
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
	x: i32,
	y: i32
}

fn print(head: &Position, tails: &[Position], visited: &HashSet<Position>) {
	println!();

	let ymin = tails
		.iter()
		.map(|pos| pos.y)
		.min()
		.unwrap()
		.min(head.y)
		.min(0);
	let ymax = tails
		.iter()
		.map(|pos| pos.y)
		.max()
		.unwrap()
		.max(head.y)
		.max(0);
	for y in -15 ..= 5 {
		let xmin = tails
			.iter()
			.map(|pos| pos.x)
			.min()
			.unwrap()
			.min(head.x)
			.min(0);
		let xmax = tails
			.iter()
			.map(|pos| pos.x)
			.max()
			.unwrap()
			.max(head.x)
			.max(0);
		for x in -11 ..= 14 {
			if head.x == x && head.y == y {
				print!("H");
			} else if let Some(i) = tails.iter().rposition(|pos| pos.x == x && pos.y == y)
			{
				if i == tails.len() - 1 {
					print!("T");
				} else {
					print!("{}", i + 1);
				}
			} else if visited.contains(&Position { x, y }) {
				print!("#");
			} else if x == 0 && y == 0 {
				print!("s");
			} else {
				print!(".");
			}
		}
		println!();
	}
}

fn run<const TAILS: usize>(moves: &[Move]) -> usize {
	let mut head = Position::default();
	let mut tails = [Position::default(); TAILS];

	let mut visited = HashSet::new();
	visited.insert(*tails.last().unwrap());

	for m in moves {
		if DEBUG {
			println!();
			println!("== {m:?} ==");
		}

		match m.direction {
			'R' => head.x += m.length,
			'L' => head.x -= m.length,
			'U' => head.y -= m.length,
			'D' => head.y += m.length,
			_ => unreachable!()
		}

		if DEBUG {
			print(&head, &tails, &visited);
		}

		let mut done = false;
		while !done {
			done = true;
			for i in 0 .. TAILS {
				let prev = match i {
					0 => &head,
					_ => &tails[i - 1]
				};

				let xdist = prev.x - tails[i].x;
				let xdiff = xdist.abs();

				let ydist = prev.y - tails[i].y;
				let ydiff = ydist.abs();

				if xdiff >= 2 || ydiff >= 2 {
					if xdiff >= 1 {
						tails[i].x += xdist / xdiff
					}
					if ydiff >= 1 {
						tails[i].y += ydist / ydiff
					}
				}

				if i == TAILS - 1 {
					visited.insert(tails[i]);
				}

				if xdiff > 2 || ydiff > 2 {
					done = false;
				}
			}
		}

		if DEBUG {
			print(&head, &tails, &visited);
		}
	}

	let xmin = visited.iter().map(|pos| pos.x).min().unwrap().min(0);
	let xmax = visited.iter().map(|pos| pos.x).max().unwrap().max(0);
	let ymin = visited.iter().map(|pos| pos.y).min().unwrap().min(0);
	let ymax = visited.iter().map(|pos| pos.y).max().unwrap().max(0);

	if DEBUG {
		println!();
		println!("== RESULT ==");
		println!();
		for y in ymin ..= ymax {
			for x in xmin ..= xmax {
				if visited.contains(&Position { x, y }) {
					print!("#");
				} else if x == 0 && y == 0 {
					print!("s");
				} else {
					print!(".");
				}
			}
			println!();
		}
	}

	visited.len()
}

fn main() -> anyhow::Result<()> {
	let file = BufReader::new(File::open("input.txt")?);
	let moves = file
		.lines()
		.map(|line| line?.parse())
		.collect::<anyhow::Result<Vec<Move>>>()?;

	println!("{}", run::<1>(&moves));
	println!("{}", run::<9>(&moves));

	Ok(())
}
