use anyhow::anyhow;
use bit_vec::BitVec;
use chumsky::{
	prelude::*,
	text::{digits, ident}
};
use indexmap::IndexMap;
use rayon::prelude::*;
use std::{collections::HashMap, fs};

struct Vertex {
	flow_rate: u32,
	adj: Vec<String>
}

fn parser() -> impl Parser<char, IndexMap<String, Vertex>, Error = Simple<char>> {
	let adj = ident()
		.then_ignore(just(", "))
		.repeated()
		.then(ident())
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

fn read<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<IndexMap<String, Vertex>> {
	parser().parse(fs::read_to_string(path)?).map_err(|errors| {
		anyhow!(errors
			.into_iter()
			.map(|err| err.to_string())
			.collect::<Vec<_>>()
			.join("\n"))
	})
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State<'a> {
	vertex: &'a str,
	flow_rate: u32,
	open: BitVec
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
	}
}

fn bfs<'a>(
	vertices: &'a IndexMap<String, Vertex>,
	max_open_vertices: usize,
	mut q: HashMap<State<'a>, u32>,
	mut remaining: u32
) -> HashMap<State<'a>, u32> {
	let init_q_len = q.len();
	while remaining > 0 {
		let q_len = q.len();
		println!(" remaining: {remaining}, q: {}", q.len());

		q = q
			.into_par_iter()
			.map(|(state, mut pressure)| {
				let mut next = HashMap::new();

				pressure += state.flow_rate;
				if state.open.len() == max_open_vertices {
					next.insert(state, pressure);
					return next;
				}

				if init_q_len != q_len && state.vertex == "AA" && state.flow_rate == 0 {
					//eprintln!("skipping {state:?}");
					return next;
				}

				let (vertex_idx, _, vertex) = vertices.get_full(state.vertex).unwrap();
				if !state.is_open(vertex_idx) && vertex.flow_rate > 0 {
					let mut state = state.clone();
					state.set_open(vertex_idx);
					state.flow_rate += vertex.flow_rate;
					let entry = next.entry(state).or_default();
					*entry = pressure.max(*entry);
				}

				for v in &vertex.adj {
					let mut state = state.clone();
					state.vertex = v;
					let entry = next.entry(state).or_default();
					*entry = pressure.max(*entry);
				}

				next
			})
			.reduce(HashMap::new, |mut acc, other| {
				for (state, pressure) in other {
					let entry = acc.entry(state).or_default();
					*entry = pressure.max(*entry);
				}
				acc
			});
		remaining -= 1;
	}
	q
}

fn main() -> anyhow::Result<()> {
	let vertices = read("input.txt")?;
	let max_open_vertices = vertices.values().filter(|v| v.flow_rate > 0).count();

	let mut q = HashMap::new();
	q.insert(
		State {
			vertex: "AA",
			flow_rate: 0,
			open: Default::default()
		},
		0
	);
	let q = bfs(&vertices, max_open_vertices, q, 30);
	let max = q.par_iter().map(|(_, pressure)| pressure).max().unwrap();
	println!("{max}");

	// part 2

	let mut q = HashMap::new();
	q.insert(
		State {
			vertex: "AA",
			flow_rate: 0,
			open: Default::default()
		},
		0
	);
	let q = bfs(&vertices, max_open_vertices, q, 26);

	let mut elephant_q = HashMap::new();
	for (state, pressure) in q {
		let key = State {
			vertex: "AA",
			flow_rate: 0,
			open: state.open
		};
		let value: &mut u32 = elephant_q.entry(key).or_default();
		*value = pressure.max(*value);
	}
	let q = bfs(&vertices, max_open_vertices, elephant_q, 26);
	let max = q.par_iter().map(|(_, pressure)| pressure).max().unwrap();
	println!("{max}");

	Ok(())
}
