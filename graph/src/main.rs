#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt::{Display,Formatter,Result};
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin};

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
	let mut g = Graph::new();
	let mut v = Vec::new();
	v.push("a".to_string());
	v.push("b".to_string());
	v.push("c".to_string());
	g.add_nodes(&mut v);
	// read_input_into_graph(stdin(), g);
	g.print_edge(stdout());
}

fn read_input_into_graph<R: Read>(reader: R, mut graph: Graph) {
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
			writeln!(writer, "{}", n);
		}
	}
	// pub fn bfs(&mut self) -> Vec<Node> {

	// }
}

impl Display for Node {
	fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Name: {} -- ", self.name);
        write!(f, "neighbors: ");
        for n in self.neighbors.iter() {
        	write!(f, "{} ", n);
        }
        Ok(())
    }
}
