/**
* graph
* an implementation of a graph data structure
* Reads in text from file to construct a graph,
* then takes two nodes (src and dest) via stdin
* to print a path between them
*
* Assumptions
* 1) Assume valid input
*	 For graph file
*		a) First word names some node in the graph, the remaining words enumerate its neighbors
*		b) Every node mentioned as a neighbor must start a line
*		c) No node may start more than one line
*       d) The second instance of any duplicate node overwrites the previous instance
*	 For stdin
*		a) A query consists of two node names, a starting node (src) and an ending node (dest)
* 		b) If a node is not in the graph, program will output that path does not exist
* 2) EOF stops the program (cmd+d on Mac)
*/

use std::io::{Read,BufReader,BufRead,stdout,Write,stdin,Result};
use std::env;
use std::fs::File;
use std::collections::{HashSet, HashMap};

pub struct Graph {
	nodes: HashMap<String, Node>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
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
		Ok(graph) => {
			read_input(stdin(), &graph);
		},
		Err(e) => println!("error! {}", e),
	}
}

fn read_graph(filename: &str) -> Result<Graph>{
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

fn read_input<R: Read> (reader: R, graph: &Graph) {
	let mut lines = BufReader::new(reader).lines();

	while let Some(Ok(line)) = lines.next() {
		let inputs_iter = line.trim().split_whitespace();
		let mut inputs = vec![];
		for input in inputs_iter {
			inputs.push(input);
		}
		if inputs.len() != 2 {
			println!("please enter nodes in the following format: src dst");
			continue;
		}
		let src = inputs.get(0).unwrap();
		let dst = inputs.get(1).unwrap();
		let path = graph.bfs(src, dst);
		graph.print_path(stdout(), &path);

	}
}

impl Graph {

	pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    // TODO: handle duplicates
	pub fn add_nodes(&mut self, new_nodes_str: &mut Vec<String>) {
		let rest;
		if new_nodes_str.len() == 0 {
			return;
		} else {
			rest = new_nodes_str.split_off(1);
		}
		let n = Node {
			name: new_nodes_str[0].clone(),
			neighbors: rest,
		};
		self.nodes.insert(new_nodes_str[0].clone(), n);
		new_nodes_str.pop();
	}

	pub fn find_node(&self, find: &str) -> Option<&Node> {
		self.nodes.get(find)
	}

	pub fn bfs(&self, from: &str, to: &str) -> Vec<String> {

		let mut queue = Vec::new();
		let mut visited = HashSet::<String>::new();
		let mut init_path = Vec::new();
		init_path.push(from.to_string());
		queue.push(init_path);

		while !queue.is_empty() {
			let curr_path_option = queue.pop();
			if curr_path_option.is_none() { continue; }

			let mut curr_path = curr_path_option.unwrap();
			let curr_node_name = curr_path.last().unwrap().clone();

			if curr_node_name == to {
				return curr_path;
			}

			let node_option = self.find_node(curr_node_name.as_str());
			if node_option.is_none() { continue; }

			let node = node_option.unwrap();
			visited.insert(curr_node_name);
			for neighbor in node.neighbors.iter() {
				if !visited.contains(neighbor) {
					curr_path.push(neighbor.clone().to_string());
					queue.push(curr_path.clone());
					curr_path.pop();
				}
			}
		}
		vec![]
	}

	pub fn print_find_node<W: Write>(&self, mut writer: W, found: &Option<&Node>){
		let f = found.unwrap();
		writeln!(writer, "{}", f.name);
	}

	pub fn print_path<W: Write>(&self, mut writer: W, path: &Vec<String>) {
		if path.len() == 0 {
			writeln!(writer, "no path!");
			return;
		}
		for n in path.iter() {
			write!(writer, "{} ", n);
		}
		writeln!(writer,"");
	}

}

#[cfg(test)]
mod add_node_tests {
	use super::{Graph, Node};
	use std::collections::HashMap;
	
	#[test]
	fn new_test() {
		let graph = Graph::new();
		assert_eq!(graph.nodes.len(), 0);
	}
	#[test]
	fn add_0_nodes() {
		let hm = HashMap::new();
		add_1_nodes_test_helper(&mut vec![], &hm);

	}

	#[test]
	fn add_1_nodes_no_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), neighbors: vec![]});
		add_1_nodes_test_helper(&mut vec!["a".to_string()], &hm);

	}

	#[test]
	fn add_1_nodes_1_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), neighbors: vec!["b".to_string()]});
		add_1_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string()], &hm);
	}

	#[test]
	fn add_1_nodes_2_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), neighbors: vec!["b".to_string(), "c".to_string()]});
		add_1_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string(), "c".to_string()], 
			&hm);
	}

	#[test]
	fn add_2_nodes_0_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), neighbors: vec![]});
		hm.insert("b".to_string(), Node {name:"b".to_string(), neighbors: vec![]});
		add_2_nodes_test_helper(&mut vec!["a".to_string()], &mut vec!["b".to_string()], &hm);
	}

	#[test]
	fn add_2_nodes_half_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), neighbors: vec!["b".to_string()]});
		hm.insert("b".to_string(), Node {name:"b".to_string(), neighbors: vec![]});
		add_2_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string()], &mut vec!["b".to_string()], &hm);
	}

	#[test]
	fn add_2_nodes_1_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), neighbors: vec!["b".to_string()]});
		hm.insert("b".to_string(), Node {name:"b".to_string(), neighbors: vec!["a".to_string()]});
		add_2_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string()], &mut vec!["b".to_string(), "a".to_string()], &hm);
	}

	#[test]
	fn add_2_nodes_2_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), neighbors: vec!["b".to_string(), "c".to_string()]});
		hm.insert("b".to_string(), Node {name:"b".to_string(), neighbors: vec!["a".to_string(), "c".to_string()]});
		add_2_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string(), "c".to_string()], 
			&mut vec!["b".to_string(), "a".to_string(), "c".to_string()], &hm);
	}

	fn add_1_nodes_test_helper(mut input: &mut Vec<String>, expected_nodes: &HashMap<String, Node>) {
		let mut g = Graph::new();
		g.add_nodes(&mut input);
		assert_eq!(g.nodes.len(), expected_nodes.len());
		assert_eq!(g.nodes, *expected_nodes);
	}

	fn add_2_nodes_test_helper(mut input: &mut Vec<String>,mut input_2: &mut Vec<String>, expected_nodes: &HashMap<String, Node>) {
		let mut g = Graph::new();
		g.add_nodes(&mut input);
		g.add_nodes(&mut input_2);
		assert_eq!(g.nodes.len(), expected_nodes.len());
		assert_eq!(g.nodes, *expected_nodes);
	}
}

#[cfg(test)]
mod find_node_tests {
	use super::{Graph, Node};
	use std::collections::HashMap;

	#[test]
	fn exist_test() {
		find_node_helper("a".to_string(), Some(&Node {name:"a".to_string(), neighbors: vec![]}));
	}
	
	#[test]
	fn not_exist_test() {
		find_node_helper("b".to_string(), None);

	}
	
	fn find_node_helper(input: String, expected_out: Option<&Node>) {
		let mut graph = Graph::new();
		graph.add_nodes(&mut vec!["a".to_string()]);

		assert_eq!(graph.find_node(&input), expected_out);
	}

}