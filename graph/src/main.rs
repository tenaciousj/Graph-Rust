#![allow(dead_code)]
use std::collections::HashMap;
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin,Result};
use std::env;
use std::fs::File;


pub struct Graph {
	nodes: Vec<Node>,
}

// pub type Node = Option<Box<NodeData>>;
#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Hash)]
pub struct Node {
	name: String,
	neighbors: Vec<String>,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("usage: graph graph.dat");
		return
	}
	let graph_file = &args[1];
	let graph_result = read_graph(&graph_file);
	match graph_result {
		Ok(g) => {
			let mut graph;
			graph = g; 
			graph.print_edge(stdout());
		},
		Err(e) => panic!("error! {}", e),
	}
	// read_input_into_graph(stdin(), g);}
}

fn read_graph(filename: &str) ->Result<Graph>{
	let file = File::open(filename)?;

	let mut g = Graph::new();
	let mut nodes = vec![];

	let mut lines = BufReader::new(file).lines();
	while let Some(Ok(line)) = lines.next() {
		let split_line = line.trim().split_whitespace();
		for word in split_line {
			nodes.push(word.to_string());
		}
		g.add_nodes(&mut nodes);
	}
	Ok(g)
}

fn read_input_into_graph<R: Read>(reader: R) {
	let mut nodes: Vec<Node> = vec![];
	let mut lines = BufReader::new(reader).lines();

	// while let Some(Ok(line)) = lines.next() {
	// 	let node_names = line.split(" ");
	// 	for n in node_names {
	// 		nodes.push(Node { name: n.to_string(), });
	// 	}
	// 	graph.add_nodes(nodes.drain(..).collect());
	// }
}

impl Graph {

	pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
        }
    }

	pub fn add_nodes(&mut self, new_nodes_str: &mut Vec<String>) {
		let rest = new_nodes_str.split_off(1);
		let n = Node {
			name: new_nodes_str[0].clone(),
			neighbors: rest,
		};
		self.nodes.push(n);
		new_nodes_str.pop();
	}

	fn find_node(&mut self, find: &str) -> Option<&Node> {
		for n in self.nodes.iter() {
			if n.name == find {
				return Some(n);
			}
		}
		None
	}
	pub fn print_edge<W: Write>(&mut self, mut writer: W) {
		for n in self.nodes.iter() {
			write!(writer, "Node: {}\nNeighbors: ", n.name);
			for neighbor in n.neighbors.iter() {
				write!(writer, "{}, ", neighbor);
			}
			write!(writer, "\n");
		}
	}
	// pub fn bfs(&mut self) -> Vec<Node> {

	// }
}