use aoc22::read;
use chumsky::{prelude::*, text::digits};
use std::{
	collections::{HashMap, HashSet, VecDeque},
	fmt::{self, Debug, Formatter},
	hash::{Hash, Hasher},
	ops::{Add, AddAssign, Sub, SubAssign}
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Resource {
	Ore,
	Clay,
	Obsidian,
	Geode
}

impl Resource {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> + Clone {
		choice((
			just("ore").map(|_| Self::Ore),
			just("clay").map(|_| Self::Clay),
			just("obsidian").map(|_| Self::Obsidian),
			just("geode").map(|_| Self::Geode)
		))
	}
}

#[derive(Debug)]
struct Blueprint {
	id: u64,
	factory: HashMap<Resource, ResourceMap>
}

impl Blueprint {
	fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
		let whitespace = just("\n  ").or(just(" ")).ignored();
		let resource = digits(10)
			.then_ignore(just(" "))
			.then(Resource::parser())
			.map(|(qty, res): (String, Resource)| (res, qty.parse().unwrap()));
		just("Blueprint ")
			.ignore_then(digits(10))
			.then_ignore(just(":"))
			.then(
				whitespace
					.ignore_then(just("Each "))
					.ignore_then(Resource::parser())
					.then_ignore(just(" robot costs "))
					.then(
						resource
							.clone()
							.then_ignore(just(" and "))
							.repeated()
							.then(resource)
							.map(|(mut vec, last)| {
								vec.push(last);
								vec
							})
					)
					.then_ignore(just("."))
					.repeated()
			)
			.map(|(id, robots): (String, _)| Self {
				id: id.parse().unwrap(),
				factory: robots
					.into_iter()
					.map(|(res, cost)| (res, cost.into()))
					.collect()
			})
	}
}

fn parser() -> impl Parser<char, Vec<Blueprint>, Error = Simple<char>> {
	Blueprint::parser()
		.then_ignore(just("\n").repeated().at_least(1))
		.repeated()
		.at_least(1)
		.then_ignore(end())
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct ResourceMap {
	ore: u64,
	clay: u64,
	obsidian: u64,
	geode: u64
}

impl From<Vec<(Resource, u64)>> for ResourceMap {
	fn from(cost: Vec<(Resource, u64)>) -> Self {
		let mut map = Self::default();
		for (res, qty) in cost {
			match res {
				Resource::Ore => map.ore += qty,
				Resource::Clay => map.clay += qty,
				Resource::Obsidian => map.obsidian += qty,
				Resource::Geode => map.geode += qty
			}
		}
		map
	}
}

impl Add for ResourceMap {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			ore: self.ore + rhs.ore,
			clay: self.clay + rhs.clay,
			obsidian: self.obsidian + rhs.obsidian,
			geode: self.geode + rhs.geode
		}
	}
}

impl AddAssign for ResourceMap {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub for ResourceMap {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			ore: self.ore - rhs.ore,
			clay: self.clay - rhs.clay,
			obsidian: self.obsidian - rhs.obsidian,
			geode: self.geode - rhs.geode
		}
	}
}

impl SubAssign for ResourceMap {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl ResourceMap {
	fn ge(self, rhs: Self) -> bool {
		self.ore >= rhs.ore
			&& self.clay >= rhs.clay
			&& self.obsidian >= rhs.obsidian
			&& self.geode >= rhs.geode
	}
}

#[derive(Clone)]
struct State<'a> {
	blueprint: &'a Blueprint,
	parent: Option<Box<State<'a>>>,
	robots: ResourceMap,
	resources: ResourceMap,
	remaining: u64
}

impl Debug for State<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		if let Some(parent) = self.parent.as_ref() {
			parent.fmt(f)?;
			f.write_str("\n")?;
		}
		writeln!(f, "== Minute {} ==", 24 - self.remaining)?;
		if self.robots.ore > 0 {
			writeln!(
				f,
				"{} ore-collecting robot(s) collect ore; you now have {} ore",
				self.robots.ore, self.resources.ore
			)?;
		}
		if self.robots.clay > 0 {
			writeln!(
				f,
				"{} clay-collecting robot(s) collect clay; you now have {} clay",
				self.robots.clay, self.resources.clay
			)?;
		}
		if self.robots.obsidian > 0 {
			writeln!(f, "{} obsidian-collecting robot(s) collect obsidian; you now have {} obsidian", self.robots.obsidian, self.resources.obsidian)?;
		}
		if self.robots.geode > 0 {
			writeln!(
				f,
				"{} geode-cracking robot(s) crack geodes; you now have {} open geode(s)",
				self.robots.geode, self.resources.geode
			)?;
		}
		Ok(())
	}
}

impl PartialEq for State<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.robots == other.robots
			&& self.resources == other.resources
			&& self.remaining == other.remaining
	}
}

impl Eq for State<'_> {}

impl Hash for State<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.robots.hash(state);
		self.resources.hash(state);
		self.remaining.hash(state);
	}
}

impl<'a> State<'a> {
	fn new(blueprint: &'a Blueprint) -> Self {
		Self {
			blueprint,
			parent: None,
			robots: vec![(Resource::Ore, 1)].into(),
			resources: ResourceMap::default(),
			remaining: 24
		}
	}
}

struct Queue<'a> {
	q: VecDeque<State<'a>>,
	inserted: HashSet<State<'a>>
}

impl<'a> Queue<'a> {
	fn new(initial: State<'a>) -> Self {
		let mut q = VecDeque::new();
		q.push_back(initial);
		Self {
			q,
			inserted: HashSet::new()
		}
	}

	fn insert(&mut self, state: State<'a>) {
		if self.inserted.contains(&state) {
			return;
		}
		let mut parentless = state.clone();
		parentless.parent = None;
		self.inserted.insert(parentless);
		self.q.push_back(state);
	}
}

fn build_robots<'a>(q: &mut Queue<'a>, state: State<'a>, resources: ResourceMap) {
	for (res, cost) in &state.blueprint.factory {
		let mut state = state.clone();
		if !resources.ge(*cost) {
			continue;
		}
		let resources = resources - *cost;
		state.resources -= *cost;
		match res {
			Resource::Ore => state.robots.ore += 1,
			Resource::Clay => state.robots.clay += 1,
			Resource::Obsidian => state.robots.obsidian += 1,
			Resource::Geode => state.robots.geode += 1
		}
		//build_robots(q, state, resources);
		q.insert(state);
	}
	q.insert(state);
}

fn bfs(initial: State<'_>) -> State<'_> {
	let mut q = Queue::new(initial.clone());
	let mut best = initial;
	let mut last = 100;
	while let Some(mut state) = q.q.pop_front() {
		if state.remaining < last {
			last = state.remaining;
			println!("remaining: {last} (q: {})", q.q.len() + 1);
			q.inserted.retain(|state| state.remaining < last);
		}

		let resources = state.resources;
		state.resources += state.robots;
		state.remaining -= 1;

		if state.resources.geode > best.resources.geode {
			best = state.clone();
		}
		if state.remaining == 0 {
			continue;
		}

		let mut max_geodes = state.resources.geode;
		// these geodes will be mined
		max_geodes += state.robots.geode * state.remaining;
		// these could be mined if we added one robot per turn
		max_geodes += state.remaining * (state.remaining + 1) / 2;
		// throw away if guaranteed bad run
		if best.resources.geode >= max_geodes {
			continue;
		}

		state.parent = Some(Box::new(state.clone()));
		build_robots(&mut q, state, resources);
	}
	best
}

fn main() -> anyhow::Result<()> {
	let blueprints = read("input.txt", parser())?;

	let mut total: u64 = 0;
	for blueprint in &blueprints {
		let state = bfs(State::new(blueprint));
		println!(
			"Blueprint {} has mined {} geodes",
			blueprint.id, state.resources.geode
		);
		println!("{state:?}");
		total += blueprint.id * state.resources.geode;
	}
	println!("{total}");

	Ok(())
}
