use anyhow::anyhow;
use chumsky::{prelude::*, text::digits};
use std::{
	collections::BTreeMap,
	fmt::{self, Display, Formatter},
	fs
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
	x: usize,
	y: usize
}

impl Position {
	fn new(x: usize, y: usize) -> Self {
		Self { x, y }
	}

	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		digits(10).then_ignore(just(",")).then(digits(10)).map(
			|(x, y): (String, String)| Self {
				x: x.parse().unwrap(),
				y: y.parse().unwrap()
			}
		)
	}
}

#[derive(Debug)]
struct Path {
	positions: Vec<Position>
}

impl Path {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		Position::parser()
			.then_ignore(just(" -> "))
			.repeated()
			.then(Position::parser())
			.map(|(mut positions, last)| {
				positions.push(last);
				Self { positions }
			})
	}
}

fn parser() -> impl Parser<char, Vec<Path>, Error = Simple<char>> {
	Path::parser()
		.then_ignore(just("\n"))
		.repeated()
		.at_least(1)
		.then_ignore(end())
}

fn read<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Vec<Path>> {
	parser().parse(fs::read_to_string(path)?).map_err(|errors| {
		anyhow!(errors
			.into_iter()
			.map(|err| err.to_string())
			.collect::<Vec<_>>()
			.join("\n"))
	})
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
	#[default]
	Air,
	Rock,
	Sand,
	Spawner
}

impl Display for Tile {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Air => ".",
			Self::Rock => "#",
			Self::Sand => "o",
			Self::Spawner => "+"
		})
	}
}

struct Map {
	map: BTreeMap<usize, Vec<Tile>>,
	min_x: usize,
	max_x: usize,
	height: usize,
	floor: Option<usize>
}

impl Map {
	fn new() -> Self {
		Self {
			map: BTreeMap::new(),
			min_x: usize::MAX,
			max_x: usize::MIN,
			height: 0,
			floor: None
		}
	}

	fn insert(&mut self, x: usize, y: usize, tile: Tile) {
		self.min_x = self.min_x.min(x);
		self.max_x = self.max_x.max(x);
		self.height = self.height.max(y + 1);

		let column = self.map.entry(x).or_default();
		if column.len() <= y {
			column.resize(y + 1, Tile::default());
		}
		column[y] = tile;
	}

	fn get(&self, x: usize, y: usize) -> Tile {
		match self.floor {
			Some(floor) if y >= floor => Tile::Rock,
			_ => self
				.map
				.get(&x)
				.and_then(|column| column.get(y))
				.copied()
				.unwrap_or_default()
		}
	}

	fn count_sand(&self) -> usize {
		self.map
			.values()
			.flat_map(|column| column.iter())
			.filter(|tile| **tile == Tile::Sand)
			.count()
	}

	fn clear_sand(&mut self) {
		for column in self.map.values_mut() {
			for tile in column.iter_mut() {
				if *tile == Tile::Sand {
					*tile = Tile::Air;
				}
			}
		}
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		for y in 0 .. self.height {
			for x in self.min_x ..= self.max_x {
				write!(f, "{}", self.get(x, y))?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

fn simulate_sand(map: &mut Map, spawner: Position, maxheight: usize) {
	let mut sand = spawner;
	while sand.y < maxheight {
		if sand != spawner {
			map.insert(sand.x, sand.y, Tile::Air);
		}

		let mut moved = false;
		if map.get(sand.x, sand.y + 1) == Tile::Air {
			sand.y += 1;
			moved = true;
		} else if map.get(sand.x - 1, sand.y + 1) == Tile::Air {
			sand.x -= 1;
			sand.y += 1;
			moved = true;
		} else if map.get(sand.x + 1, sand.y + 1) == Tile::Air {
			sand.x += 1;
			sand.y += 1;
			moved = true;
		}
		map.insert(sand.x, sand.y, Tile::Sand);

		if !moved {
			if sand == spawner {
				break;
			}
			sand = spawner;
		}
	}
}

fn simulate_sand_fast(map: &mut Map, spawner: Position) {
	let mut q = vec![spawner];
	while let Some(mut sand) = q.last().copied() {
		let mut moved = false;
		if map.get(sand.x, sand.y + 1) == Tile::Air {
			sand.y += 1;
			moved = true;
		} else if map.get(sand.x - 1, sand.y + 1) == Tile::Air {
			sand.x -= 1;
			sand.y += 1;
			moved = true;
		} else if map.get(sand.x + 1, sand.y + 1) == Tile::Air {
			sand.x += 1;
			sand.y += 1;
			moved = true;
		}
		if moved {
			q.push(sand);
		} else {
			map.insert(sand.x, sand.y, Tile::Sand);
			q.remove(q.len() - 1);
		}
	}
}

fn main() -> anyhow::Result<()> {
	let paths = read("input.txt")?;
	let mut map = Map::new();
	for path in paths {
		let mut last: Option<Position> = None;
		for pos in path.positions {
			if let Some(last) = last {
				for x in last.x.min(pos.x) ..= last.x.max(pos.x) {
					for y in last.y.min(pos.y) ..= last.y.max(pos.y) {
						map.insert(x, y, Tile::Rock);
					}
				}
			}
			last = Some(pos);
		}
	}

	let spawner = Position::new(500, 0);
	map.insert(spawner.x, spawner.y, Tile::Spawner);
	println!("{map}");

	// part 1
	let maxheight = map.height;
	simulate_sand(&mut map, spawner, maxheight);
	println!("{map}");
	println!("{}", map.count_sand() - 1);

	// part 2
	map.clear_sand();
	map.floor = Some(maxheight + 1);
	simulate_sand_fast(&mut map, spawner);
	println!("{map}");
	println!("{}", map.count_sand());

	Ok(())
}
