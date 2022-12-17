use anyhow::anyhow;
use chumsky::prelude::*;
use std::{fs, path::Path};

pub fn read<P, C, T>(path: P, parser: C) -> anyhow::Result<T>
where
	P: AsRef<Path>,
	C: Parser<char, T, Error = Simple<char>>
{
	parser.parse(fs::read_to_string(path)?).map_err(|errors| {
		anyhow!(errors
			.into_iter()
			.map(|err| err.to_string())
			.collect::<Vec<_>>()
			.join("\n"))
	})
}
