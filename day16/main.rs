use aoc22::read;
use bit_vec::BitVec;
use chumsky::{
	prelude::*,
	text::{digits, ident}
};
use indexmap::IndexMap;
use std::{
	collections::HashMap,
	hash::{Hash, Hasher}
};

#[derive(Debug)]
struct Vertex {
	flow_rate: u32,
	adj: Vec<Edge>
}

#[derive(Debug)]
struct Edge {
	time: u32,
	next: String
}

fn parser() -> impl Parser<char, IndexMap<String, Vertex>, Error = Simple<char>> {
	let edge = ident().map(|next| Edge { time: 1, next });
	let adj =
		edge.then_ignore(just(", "))
			.repeated()
			.then(edge)
			.map(|(mut adj, last)| {
				adj.push(last);
				adj
			});
	let vertex = just("Valve ")
		.ignore_then(ident())
		.then_ignore(just(" has flow rate="))
		.then(digits(10).map(|digits: String| digits.parse().unwrap()))
		.then_ignore(
			just("; tunnels lead to valves ").or(just("; tunnel leads to valve "))
		)
		.then(adj)
		.map(|((id, flow_rate), adj)| (id, Vertex { flow_rate, adj }));
	vertex
		.then_ignore(just("\n"))
		.repeated()
		.at_least(1)
		.then_ignore(end())
		.map(|vec| vec.into_iter().collect())
}

#[derive(Clone, Debug, Eq)]
struct State<'a> {
	vertex: &'a str,
	flow_rate: u32,
	open: BitVec,
	open_count: usize
}

// PERFORMANCE HACK: We only consider vertex and open valves for the PartialEq and Hash
// implementations. This makes no difference for the human run, but has a huge implication
// on the elephant run: The flow rate can differ between "equal" states. However, due to
// the way the HashMap works, this greatly reduces the amount of steps needed without
// influencing the result.

impl PartialEq for State<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.vertex == other.vertex && self.open == other.open
	}
}

impl Hash for State<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.vertex.hash(state);
		self.open.hash(state);
	}
}

impl State<'_> {
	fn is_open(&self, idx: usize) -> bool {
		self.open.get(idx).unwrap_or(false)
	}

	fn set_open(&mut self, idx: usize) {
		if self.open.len() <= idx {
			self.open.grow(idx - self.open.len() + 1, false);
		}
		self.open.set(idx, true);
		self.open_count += 1;
	}
}

fn bfs<'a>(
	vertices: &'a IndexMap<String, Vertex>,
	max_open_vertices: usize,
	q: HashMap<State<'a>, u32>,
	mut remaining: u32
) -> HashMap<State<'a>, u32> {
	let init_q_len = q.len();
	let mut qs = HashMap::new();
	qs.insert(remaining, q);
	for i in 0 .. remaining {
		qs.insert(i, Default::default());
	}

	while remaining > 0 {
		let q = qs.remove(&remaining).unwrap();
		let q_len = q.len();
		println!(" remaining: {remaining}, q: {}", q.len());

		for (state, pressure) in q {
			if state.open_count == max_open_vertices {
				let pressure = pressure + state.flow_rate;
				qs.get_mut(&(remaining - 1))
					.unwrap()
					.insert(state, pressure);
				continue;
			}

			if init_q_len != q_len && state.vertex == "AA" && state.flow_rate == 0 {
				//eprintln!("skipping {state:?}");
				continue;
			}

			let (vertex_idx, _, vertex) = vertices.get_full(state.vertex).unwrap();
			if !state.is_open(vertex_idx) && vertex.flow_rate > 0 {
				let pressure = pressure + state.flow_rate;
				let mut state = state.clone();
				state.set_open(vertex_idx);
				state.flow_rate += vertex.flow_rate;
				let entry = qs
					.get_mut(&(remaining - 1))
					.unwrap()
					.entry(state)
					.or_default();
				*entry = pressure.max(*entry);
			}

			for edge in &vertex.adj {
				let travel = edge.time.min(remaining);
				let pressure = pressure + state.flow_rate * travel;
				let mut state = state.clone();
				state.vertex = &edge.next;
				let entry = qs
					.get_mut(&(remaining - travel))
					.unwrap()
					.entry(state)
					.or_default();
				*entry = pressure.max(*entry);
			}
		}
		remaining -= 1;
	}

	qs.remove(&0).unwrap()
}

fn inline_edge(v: &str, vertex: &mut Vertex, edge: &Edge) {
	vertex.adj.iter_mut().filter(|e| e.next == v).for_each(|e| {
		e.time += edge.time;
		e.next = edge.next.to_owned();
	});
}

fn main() -> anyhow::Result<()> {
	let mut vertices = read("input.txt", parser())?;

	for v in vertices.keys().cloned().collect::<Vec<_>>() {
		let vertex = &vertices[&v];
		if vertex.flow_rate > 0 || vertex.adj.len() > 2 {
			continue;
		}
		let vertex = vertices.remove(&v).unwrap();
		match vertex.adj.len() {
			0 => {},
			1 => vertices
				.get_mut(&vertex.adj.first().unwrap().next)
				.unwrap()
				.adj
				.retain(|edge| edge.next != v),
			2 => {
				let first = vertex.adj.first().unwrap();
				let second = vertex.adj.last().unwrap();
				inline_edge(&v, vertices.get_mut(&first.next).unwrap(), second);
				inline_edge(&v, vertices.get_mut(&second.next).unwrap(), first);
			},
			_ => unreachable!()
		}
	}
	for (key, v) in &vertices {
		println!("{key}:\t{v:?}");
	}

	let max_open_vertices = vertices.values().filter(|v| v.flow_rate > 0).count();

	let mut q = HashMap::new();
	q.insert(
		State {
			vertex: "AA",
			flow_rate: 0,
			open: Default::default(),
			open_count: 0
		},
		0
	);
	let q = bfs(&vertices, max_open_vertices, q, 30);
	let max = q.iter().map(|(_, pressure)| pressure).max().unwrap();
	println!("{max}");

	// part 2

	let mut q = HashMap::new();
	q.insert(
		State {
			vertex: "AA",
			flow_rate: 0,
			open: Default::default(),
			open_count: 0
		},
		0
	);
	let q = bfs(&vertices, max_open_vertices, q, 26);

	let mut elephant_q = HashMap::new();
	for (state, pressure) in q {
		let key = State {
			vertex: "AA",
			flow_rate: 0,
			open: state.open,
			open_count: state.open_count
		};
		let value: &mut u32 = elephant_q.entry(key).or_default();
		*value = pressure.max(*value);
	}
	let q = bfs(&vertices, max_open_vertices, elephant_q, 26);
	let max = q.iter().map(|(_, pressure)| pressure).max().unwrap();
	println!("{max}");

	Ok(())
}
