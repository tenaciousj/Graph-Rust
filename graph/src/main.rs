#![allow(dead_code)]
use std::fmt::{Display,Formatter,Result};
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin};

pub struct Graph {
	nodes: Vec<Node>,
}

#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Hash)]
pub struct Node {
	name: String,
	neighbors: Vec<String>,
}

fn main() {
	let mut g = Graph::new();
	//pooooooooooooooooooop
	// let mut v = Vec::new();
	// v.push("a".to_string());
	// v.push("b".to_string());
	// v.push("c".to_string());
	// g.add_nodes(&mut v);
	// g.print_find_node(stdout(), &g.find_node("a"));

	// let path = g.bfs("a", "b");
	// g.print_path(stdout(), &path);
}

// fn read_input_into_graph<R: Read>(reader: R, mut graph: Graph) {
// 	let mut nodes: Vec<Node> = vec![];
// 	let mut lines = BufReader::new(reader).lines();

// 	while let Some(Ok(line)) = lines.next() {
// 		let node_names = line.split(" ");
// 		for n in node_names {
// 			nodes.push(Node { name: n.to_string(), });
// 		}
// 		graph.add_nodes(nodes.drain(..).collect());
// 	}
// }




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

	pub fn find_node(&self, find: &str) -> Option<&Node> {
		for n in self.nodes.iter() {
			if n.name == find {
				return Some(n);
			}
		}
		None
	}

	pub fn bfs(&mut self, from: &str, to: &str) -> Vec<String> {
		let mut queue = Vec::new();
		let mut visited = Vec::<&String>::new();
		let mut path = Vec::<String>::new();

		queue.push(from);

		while !queue.is_empty() {
			let node_name = queue.pop().unwrap();
			path.push(node_name.to_string());
			if node_name == to {
				return path;
			}
			let node = self.find_node(node_name).unwrap();
			for neighbor in node.neighbors.iter() {
				if !visited.contains(&neighbor) {
					visited.push(&neighbor);
					queue.push(neighbor);
				}
			}

		}
		vec![]

	}


	pub fn print_find_node<W: Write>(&self, mut writer: W, found: &Option<&Node>){
		let f = found.unwrap();
		writeln!(writer, "{}", f.name);
	}


	pub fn print_path<W: Write>(&mut self, mut writer: W, path: &Vec<String>) {
		for n in path.iter() {
			writeln!(writer, "{}", n);
		}
	}


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
