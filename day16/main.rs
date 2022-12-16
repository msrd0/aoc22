use anyhow::anyhow;
use chumsky::{
	prelude::*,
	text::{digits, ident}
};
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
	fmt::{self, Display, Formatter},
	fs
};

struct Vertex {
	flow_rate: u32,
	adj: Vec<String>
}

fn parser() -> impl Parser<char, HashMap<String, Vertex>, Error = Simple<char>> {
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

fn read<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<HashMap<String, Vertex>> {
	parser().parse(fs::read_to_string(path)?).map_err(|errors| {
		anyhow!(errors
			.into_iter()
			.map(|err| err.to_string())
			.collect::<Vec<_>>()
			.join("\n"))
	})
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
	previous: Option<String>,
	vertex: String,
	remaining: u32,
	flow_rate: u32,
	pressure: u32,
	open: BTreeSet<String>
}

fn dfs(
	vertices: &HashMap<String, Vertex>,
	max_open_vertices: usize,
	visited: &mut HashSet<State>,
	mut state: State
) -> u32 {
	visited.insert(state.clone());
	if state.remaining == 0 {
		return state.pressure;
	}
	state.pressure += state.flow_rate;
	if state.open.len() == max_open_vertices {
		state.remaining -= 1;
		return dfs(vertices, max_open_vertices, visited, state);
	}

	let mut max = 0;
	let vertex = &vertices[&state.vertex];
	if !state.open.contains(&state.vertex) && vertex.flow_rate > 0 {
		let mut state = state.clone();
		state.previous = None;
		state.open.insert(state.vertex.clone());
		state.flow_rate += vertex.flow_rate;
		state.remaining -= 1;
		if !visited.contains(&state) {
			max = max.max(dfs(vertices, max_open_vertices, visited, state));
		}
	}
	for next in &vertex.adj {
		if state.previous.as_ref() == Some(next) {
			continue;
		}
		let mut state = state.clone();
		state.vertex = next.into();
		state.remaining -= 1;
		if !visited.contains(&state) {
			max = max.max(dfs(vertices, max_open_vertices, visited, state));
		}
	}
	max
}

fn main() -> anyhow::Result<()> {
	let vertices = read("input.txt")?;
	let max_open_vertices = vertices.values().filter(|v| v.flow_rate > 0).count();

	let mut visited = Default::default();
	let max = dfs(&vertices, max_open_vertices, &mut visited, State {
		previous: None,
		vertex: "AA".into(),
		remaining: 30,
		flow_rate: 0,
		pressure: 0,
		open: BTreeSet::new()
	});
	println!("{max}");

	Ok(())
}
