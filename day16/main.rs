use anyhow::anyhow;
use chumsky::{
	prelude::*,
	text::{digits, ident}
};
use indexmap::IndexSet;
use std::{
	collections::{BTreeSet, HashMap},
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
	vertex: String,
	flow_rate: u32,
	pressure: u32,
	open: BTreeSet<String>
}

fn main() -> anyhow::Result<()> {
	let vertices = read("input.txt")?;
	let max_open_vertices = vertices.values().filter(|v| v.flow_rate > 0).count();

	let mut remaining = 30;
	let mut q = IndexSet::new();
	let mut next = IndexSet::new();

	q.insert(State {
		vertex: "AA".into(),
		flow_rate: 0,
		pressure: 0,
		open: Default::default()
	});

	while remaining > 0 {
		println!(" remaining: {remaining}, q: {}", q.len());
		while let Some(mut state) = q.pop() {
			state.pressure += state.flow_rate;
			if state.open.len() == max_open_vertices {
				next.insert(state);
				continue;
			}

			let vertex = &vertices[&state.vertex];
			if !state.open.contains(&state.vertex) && vertex.flow_rate > 0 {
				let mut state = state.clone();
				state.open.insert(state.vertex.clone());
				state.flow_rate += vertex.flow_rate;
				next.insert(state);
			}

			for v in &vertex.adj {
				let mut state = state.clone();
				state.vertex = v.to_owned();
				next.insert(state);
			}
		}
		assert!(q.is_empty());
		q = next;
		next = IndexSet::new();
		remaining -= 1;
	}

	let max = q.iter().map(|state| state.pressure).max().unwrap();
	println!("{max}");

	Ok(())
}
