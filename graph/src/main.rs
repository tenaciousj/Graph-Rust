#![allow(dead_code)]
use std::collections::HashMap;
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin,Result};
use std::env;
use std::fs::File;

pub struct Graph {
	nodes: Vec<Node>,
}

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
	let mut graph;
	match graph_result {
		Ok(g) => {
			graph = g; 
			// graph.print_edge(stdout());

			let path = graph.bfs("a", "d");
			graph.print_path(stdout(), &path);
		},
		Err(e) => println!("error! {}", e),
	}
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

	pub fn find_node(&self, find: &str) -> Option<&Node> {
		for n in self.nodes.iter() {
			if n.name == find {
				return Some(n);
			}
		}
		None
	}

	pub fn bfs(&self, from: &str, to: &str) -> Vec<String> {
		let mut queue = Vec::<Vec<String>>::new();
		let mut visited = Vec::new();
		let mut path = Vec::new();
		path.push(from.to_string());
		queue.push(path);

		while !queue.is_empty() {
			let mut curr_path = queue.pop().unwrap();
			
			let curr_node_name = curr_path.last().unwrap().clone();
			if curr_node_name == to {
				return curr_path;
			}

			let node = self.find_node(curr_node_name.as_str()).unwrap();
			visited.push(curr_node_name);
			for neighbor in node.neighbors.iter() {
				if !visited.contains(&neighbor) {
					curr_path.push(neighbor.clone().to_string());
					queue.push(curr_path.clone());
					curr_path.pop();
				}
			}
			// let node_name = queue.pop().unwrap();
			// paths.push(node_name.to_string());
			// if node_name == to {
			// 	return paths;
			// }
			// let node = self.find_node(node_name).unwrap();
			// for neighbor in node.neighbors.iter() {
			// 	if !visited.contains(&neighbor) {
			// 		visited.push(&neighbor);
			// 		queue.push(neighbor);
			// 	}
			// }

		}
		vec![]

	}


	pub fn print_find_node<W: Write>(&self, mut writer: W, found: &Option<&Node>){
		let f = found.unwrap();
		writeln!(writer, "{}", f.name);
	}


	pub fn print_path<W: Write>(&mut self, mut writer: W, path: &Vec<String>) {
		for n in path.iter() {
			write!(writer, "{} ", n);
		}
		writeln!(writer,"");
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

}


