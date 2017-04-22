# Graph-Rust
An implementation of a Graph data structure in Rust

The purpose of graph is to find paths in graphs. It reads a graph specification from a file, and then answers routing queries typed by the user.

A graph specification file represents an undirected graph as an association list of nodes, written as tokens, In particular, each line is a list of words, where the first word names some node in the graph and the remaining words enumerate its neighbors. Every node mentioned as a neighbor must start a line as well, and no node may start more than one line.

The user enters queries on stdin, one at a time. A query consists of two node names, a starting node and an ending node. The program then prints out a path between the nodes, or a message that no such path exists.
