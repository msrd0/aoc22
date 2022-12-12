#![allow(clippy::enum_variant_names)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

use anyhow::bail;
use std::{
	collections::{HashSet, VecDeque},
	fs::File,
	io::{BufRead as _, BufReader}
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Height {
	Height(u32),
	Start,
	End
}

impl Height {
	fn height(self) -> u32 {
		match self {
			Self::Height(height) => height,
			Self::Start => 0,
			Self::End => 25
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
struct Path {
	x: usize,
	y: usize,
	steps: usize
}

fn main() -> anyhow::Result<()> {
	let heightmap = BufReader::new(File::open("input.txt")?)
		.lines()
		.map(|line| {
			line.map_err(anyhow::Error::from).and_then(|line| {
				line.chars()
					.map(|ch| {
						Ok(match ch {
							'S' => Height::Start,
							'E' => Height::End,
							'a' ..= 'z' => Height::Height(ch as u32 - 'a' as u32),
							_ => bail!("Invalid input {ch}")
						})
					})
					.collect::<Result<Vec<_>, _>>()
			})
		})
		.collect::<Result<Vec<_>, _>>()?;

	let mut visited = HashSet::new();
	let mut q = VecDeque::new();
	for (y, row) in heightmap.iter().enumerate() {
		for (x, height) in row.iter().enumerate() {
			if *height == Height::End {
				q.push_front(Path { x, y, steps: 0 });
				visited.insert((x, y));
			}
		}
	}
	'outer: while let Some(path) = q.pop_front() {
		eprintln!("[DEBUG] path={path:?}");
		for (x, y) in [
			(path.x.checked_sub(1).map(|x| (x, path.y))),
			(path.x.checked_add(1).map(|x| (x, path.y))),
			(path.y.checked_sub(1).map(|y| (path.x, y))),
			(path.y.checked_add(1).map(|y| (path.x, y)))
		]
		.into_iter()
		.flatten()
		{
			if x >= heightmap[0].len()
				|| y >= heightmap.len()
				|| visited.contains(&(x, y))
				|| heightmap[y][x].height() + 1 < heightmap[path.y][path.x].height()
			{
				continue;
			}
			//eprintln!("[DEBUG]  -> ({x}, {y})?");
			match heightmap[y][x] {
				// part 1
				//Height::Start => {
				// part 2
				height if height.height() == 0 => {
					println!(
						"It took {} steps to get from the end to the start",
						path.steps + 1
					);
					break 'outer;
				},
				Height::Height(height) => {
					q.push_back(Path {
						x,
						y,
						steps: path.steps + 1
					});
					visited.insert((x, y));
				},
				_ => {}
			}
		}
	}

	Ok(())
}
