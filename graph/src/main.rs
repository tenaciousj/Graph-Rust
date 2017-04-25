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

use std::env;
use std::fs::File;
use std::collections::{HashSet,HashMap,VecDeque};
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin,Result};

pub type NodeName = String;
pub struct Graph {
	//Vec<NodeName> is a list of node's neighbors
	nodes: HashMap<NodeName, Vec<NodeName>>,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("usage: graph graph.dat");
		return
	}
	let graph_file = &args[1];
	let graph_result = read_graph(&graph_file);

	//if graph was read successfully, accept input
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
		//add nodes to graph
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
		let path = graph.path_finder(src, dst);
		graph.print_path(stdout(), &path);

	}
}

impl Graph {

	pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }
    //TODO: make sure that a node does not add itself as neighbor

	pub fn add_nodes(&mut self, new_nodes_str: &mut Vec<NodeName>) {
		//no new nodes were inputted
		if new_nodes_str.len() == 0 {
			return;
		}
		let mut rest = new_nodes_str.split_off(1);
		//remove duplicates
		rest.sort();
		rest.dedup();

		//add neighbors to current node
		{
			let curr_name = new_nodes_str[0].clone();
			let curr = self.nodes.entry(curr_name).or_insert(Vec::new());
			curr.append(&mut rest.clone());
		}

		//for each neighbor of current node, add itself to its neighbor's "neighbors" list
		for neighbor in rest {
			let nes = self.nodes.entry(neighbor).or_insert(Vec::new());
			nes.push(new_nodes_str[0].clone());

		}
		//will use this vec again in read_graph...clear it out
		new_nodes_str.drain(..);
	}

	pub fn find_node(&self, find: &str) -> Option<&Vec<NodeName>> {
		self.nodes.get(find)
	}

	pub fn path_finder(&self, from: &str, to: &str) -> Vec<NodeName> {
		//uses breadth first search
		//deque will be a vecdeque of vecs, with each vec being a possible path
		let mut deque = VecDeque::new();
		let mut visited = HashSet::new();
		let mut init_path = Vec::new();

		init_path.push(from.to_string());
		deque.push_back(init_path);

		while !deque.is_empty() {
			while let Some(mut curr_path) = deque.pop_front() {
				//get current node
				let curr_node_name = curr_path.last().unwrap().clone();

				//if this is destination return its path
				if curr_node_name == to {
					return curr_path;
				}
				//else if the node is found in the graph
				if let Some(node_neighbors) = self.find_node(curr_node_name.as_str()) {
					visited.insert(curr_node_name);
					//repeat process with each of current node's neighbors
					for neighbor in node_neighbors {
						if !visited.contains(neighbor) {
							curr_path.push(neighbor.to_string());
							deque.push_back(curr_path.clone());
							curr_path.pop();
						}
					}
				} else { continue; }
			}
		}
		vec![]
	}


	pub fn print_path<W: Write>(&self, mut writer: W, path: &Vec<NodeName>) {
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
	use super::{Graph, NodeName};
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
		hm.insert("a".to_string(), Vec::new());
		add_1_nodes_test_helper(&mut vec!["a".to_string()], &hm);

	}

	#[test]
	fn add_1_nodes_1_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), vec!["b".to_string()]);
		add_1_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string()], &hm);
	}

	#[test]
	fn add_1_nodes_2_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), vec!["b".to_string(), "c".to_string()]);
		hm.insert("b".to_string(), vec!["a".to_string()]);
		hm.insert("d".to_string(), vec!["a".to_string()]);
		add_1_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string(), "c".to_string()], 
			&hm);
	}

	#[test]
	fn add_1_nodes_dup_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Node {name:"a".to_string(), 
			neighbors: vec!["b".to_string(), "c".to_string()]});
		add_1_nodes_test_helper(&mut vec![ "a".to_string(), "b".to_string(),
			"c".to_string(),"b".to_string() ], &hm);
	}

	#[test]
	fn add_2_nodes_0_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), Vec::new());
		hm.insert("b".to_string(), Vec::new());
		add_2_nodes_test_helper(&mut vec!["a".to_string()], &mut vec!["b".to_string()], &hm);
	}

	#[test]
	fn add_2_nodes_half_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), vec!["b".to_string()]);
		hm.insert("b".to_string(), Vec::new());
		add_2_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string()], &mut vec!["b".to_string()], &hm);
	}

	#[test]
	fn add_2_nodes_1_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), vec!["b".to_string()]);
		hm.insert("b".to_string(), vec!["a".to_string()]);
		add_2_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string()], &mut vec!["b".to_string(), "a".to_string()], &hm);
	}

	#[test]
	fn add_2_nodes_2_neighbor() {
		let mut hm = HashMap::new();
		hm.insert("a".to_string(), vec!["b".to_string(), "c".to_string()]);
		hm.insert("b".to_string(), vec!["a".to_string(), "c".to_string()]);
		add_2_nodes_test_helper(&mut vec!["a".to_string(), "b".to_string(), "c".to_string()], 
			&mut vec!["b".to_string(), "a".to_string(), "c".to_string()], &hm);
	}

	fn add_1_nodes_test_helper(mut input: &mut Vec<String>, expected_nodes: &HashMap<String, Vec<NodeName>>) {
		let mut g = Graph::new();
		g.add_nodes(&mut input);
		assert_eq!(g.nodes.len(), expected_nodes.len());
		assert_eq!(g.nodes, *expected_nodes);
	}

	fn add_2_nodes_test_helper(mut input: &mut Vec<String>,mut input_2: &mut Vec<String>, expected_nodes: &HashMap<String, Vec<NodeName>>) {
		let mut g = Graph::new();
		g.add_nodes(&mut input);
		g.add_nodes(&mut input_2);
		assert_eq!(g.nodes.len(), expected_nodes.len());
		assert_eq!(g.nodes, *expected_nodes);
	}
}

#[cfg(test)]
mod find_node_tests {
	use super::Graph;
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

#[cfg(test)] 
mod path_finder_tests {
	use super::Graph;

	// #[test] 
	
}
