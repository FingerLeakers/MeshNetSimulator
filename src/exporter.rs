
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use serde_json::Value;
use node::Node;
use sim::NodeMeta;
use sim::RoutingAlgorithm;
use graph::{Graph, ID};
use utils::*;
use std::u16;
use std::fmt;
use std::fmt::Write;


pub fn export_file(graph: &Graph, algo: Option<&RoutingAlgorithm>, path: &str) {
	use std::io::Write;
	if let Ok(mut file) = File::create(path) {
		let content = export_json(&graph, algo);
		file.write_all(content.as_bytes()).unwrap();
		println!("Wrote {}", path);
	} else {
		println!("Failed to create: {}", path);
	}
}

pub fn export_json(graph: &Graph, algo: Option<&RoutingAlgorithm>) -> String {
	let mut ret = String::new();
	let mut meta = NodeMeta::new();

	write!(&mut ret, "{{").unwrap();
	write!(&mut ret, "\"nodes\": [").unwrap();
	let mut comma1 = false;
	for (id, node) in graph.nodes.iter().enumerate() {
		if comma1 {
			write!(&mut ret, ",").unwrap();
		}
		comma1 = true;
		if let Some(algo) = algo {
			meta.clear();
			algo.get_node_meta(id as ID, &mut meta);
		}
		let x = node.gpos.x();
		let y = node.gpos.y();
		write!(&mut ret,
			"{{\"id\": \"{}\", \"x\": {}, \"y\": {}, \"name\": \"{}\", \"label\": \"{}\"}}",
			id, x, y, meta.name, meta.label).unwrap();
	}

	write!(&mut ret, "], \"links\": [").unwrap();
	let mut comma2 = false;
	//for (id, node) in graph.nodes.iter().enumerate() {
	for link in &graph.links {
		if link.from > link.to {
			continue;
		}

		if comma2 {
			write!(&mut ret, ",").unwrap();
		}
		comma2 = true;
		// how to remember
		let source_id = link.from;
		let source_tq = (link.quality() as f32) / (u16::MAX as f32);
		let target_id = link.to;
		let target_tq = if let Some(link) = graph.get_link(target_id, source_id) {
			(link.quality() as f32) / (u16::MAX as f32)
		} else {
			0.0
		};
		write!(&mut ret, "{{\"source\": \"{}\", \"target\": \"{}\", \"source_tq\": {}, \"target_tq\": {}}}",
			source_id, target_id, source_tq, target_tq
		).unwrap();
	}

	write!(&mut ret, "]}}").unwrap();

	ret
}

/*
pub fn export_netjson(graph: &Graph) -> String {
	let mut ret = String::new();

	write!(&mut ret, "{{\"type\": \"NetworkGraph\", \"protocol\": \"\", \"version\": \"\", \"metric\": \"tq\", \"directed\": false, \"multigraph\": false,").unwrap();
	//write!(&mut ret, "\"timestamp\": \"{:?}\", ", chrono::Utc::now()).unwrap();

	write!(&mut ret, "\"nodes\": [").unwrap();
	let mut comma1 = false;
	for (id, node) in graph.nodes.iter().enumerate() {
		if comma1 {
			write!(&mut ret, ",").unwrap();
		}
		comma1 = true;
		write!(&mut ret, "{{\"id\": \"{}\"}}", id).unwrap();
	}

	write!(&mut ret, "], \"links\": [").unwrap();
	let mut comma2 = false;
	for (id, node) in graph.nodes.iter().enumerate() {
		for link in &node.links {
			if comma2 {
				write!(&mut ret, ",").unwrap();
			}
			comma2 = true;
			if link.to < (id as ID) {
				// how to remember
				let source_id = id as ID;
				let source_tq = link.quality;
				let target_id = link.to;
				let target_tq = if let Some(link) = graph.nodes[link.to as usize].find_link(source_id) {
					link.quality
				} else {
					0.0
				};
				write!(&mut ret, "{{\"source\": \"{}\", \"target\": \"{}\", \"source_tq\": {}, \"target_tq\": {}}}",
					source_id, target_id, source_tq, target_tq
				).unwrap();
			}
		}
	}

	write!(&mut ret, "]}}").unwrap();

	ret
}
*/
