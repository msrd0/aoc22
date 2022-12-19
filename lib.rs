use anyhow::anyhow;
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, prelude::*};
use std::{fs, path::Path};

fn report_err(buf: &str, path_str: &str, err: Vec<Simple<char>>) {
	for e in err {
		let mut report = Report::build(ReportKind::Error, path_str, e.span().start);
		match (e.reason(), e.found()) {
			(SimpleReason::Unexpected, Some(found)) => {
				report.set_message("Unexpected token");
				report.add_label(
					Label::new((path_str, e.span()))
						.with_message(format!("Unexpected token {found}"))
				);
				if e.expected().len() > 0 {
					report.set_note(format!(
						"Expected {}",
						e.expected()
							.map(|ex| match ex {
								Some(ex) => format!("{ex:?}"),
								None => "end of file".to_owned()
							})
							.collect::<Vec<_>>()
							.join(", ")
					));
				}
			},

			(SimpleReason::Unexpected, None) => {
				report.set_message("Unexpected end of file");
			},

			(SimpleReason::Unclosed { span, delimiter }, found) => {
				report.set_message("Unclosed delimiter");
				report.add_label(
					Label::new((path_str, span.clone()))
						.with_message(format!("Unclosed delimiter {}", delimiter))
				);
				if let Some(found) = found {
					report.add_label(
						Label::new((path_str, e.span()))
							.with_message(format!("Must be closed before this {found}"))
					);
				}
			},

			(SimpleReason::Custom(msg), _) => {
				report.set_message(msg);
				report.add_label(Label::new((path_str, e.span())).with_message(msg));
			}
		};
		report
			.finish()
			.print((path_str, Source::from(buf)))
			.unwrap()
	}
}

pub fn read<P, C, T>(path: P, parser: C) -> anyhow::Result<T>
where
	P: AsRef<Path>,
	C: Parser<char, T, Error = Simple<char>>
{
	let path = path.as_ref();
	let buf = fs::read_to_string(path)?;
	parser.parse(buf.as_str()).map_err(|errors| {
		report_err(&buf, &path.to_string_lossy(), errors);
		anyhow!("Failed to parse input")
	})
}
