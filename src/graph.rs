use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use crate::graph;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Node {
    pub id: usize,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, id: usize, label: String) {
        self.nodes.push(Node { id, label });
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push(Edge { source, target });
    }

    pub fn remove_edge(&mut self, source: usize, target: usize) {
        let index = self
            .edges
            .iter()
            .position(|edge| edge.source == source && edge.target == target);
        match index {
            Some(index) => {
                self.edges.remove(index);
            }
            None => eprintln!("Edge with this index not found"),
        }
    }

    pub fn remove_node(&mut self, id: usize) {
        let index = self.nodes.iter().position(|node| node.id == id);
        match index {
            Some(index) => {
                self.nodes.remove(index);
                self.edges
                    .retain(|edge| edge.source != id && edge.target != id);
            }
            None => eprintln!("Node with this index not found"),
        }
    }

    pub fn breadth_first_search(&self, start_node_id: usize) -> Vec<usize> {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = vec![false; self.nodes.len()];
        let mut result = vec![];

        queue.push_back(start_node_id);
        visited[start_node_id] = true;
        result.push(start_node_id);

        while let Some(current_node_id) = queue.pop_front() {
            for edge in &self.edges {
                if edge.source == current_node_id {
                    let target = edge.target;
                    if !visited[target] {
                        visited[target] = true;
                        queue.push_back(target);
                        result.push(target);
                    }
                }
            }
        }
        result
    }

    pub fn depth_first_search(&self, start_node_id: usize) -> Vec<usize> {
        let mut stack = vec![start_node_id];
        let mut visited = vec![false; self.nodes.len()];
        let mut result = vec![];

        while let Some(current_node_id) = stack.pop() {
            if !visited[current_node_id] {
                visited[current_node_id] = true;
                result.push(current_node_id);
                for edge in &self.edges {
                    if edge.source == current_node_id {
                        let target = edge.target;
                        if !visited[target] {
                            stack.push(target);
                        }
                    }
                }
            }
        }
        result
    }

    pub fn from_json(input: &str) -> Self {
        serde_json::from_str(input).expect("Failed to parse JSON input")
    }

    pub fn to_json(&self, path: String) -> String {
        let json_string = serde_json::to_string(self).expect("Failed to convert struct to JSON");

        let mut file = File::create(path).expect("can't find graph.json file");
        file.write_all(json_string.as_bytes()).unwrap();
        json_string 
    }
}

mod tests {
    
    use super::*;

    #[allow(dead_code)]
    fn get_graph_from_json() -> Graph {
        let json_str = r#"
    {
        "nodes": [
            {"id": 0, "label": "A"},
            {"id": 1, "label": "B"},
            {"id": 2, "label": "C"},
            {"id": 3, "label": "D"},
            {"id": 4, "label": "E"}
        ],
        "edges": [
            {"source": 0, "target": 2},
            {"source": 1, "target": 3},
            {"source": 2, "target": 3},
            {"source": 3, "target": 4}
        ]
    }
    "#;
        graph::Graph::from_json(json_str)
    }

    #[test]
    fn test_breadth_first_search() {
        let graph = get_graph_from_json();
        let result = graph.breadth_first_search(0);
        let expected_result = vec![0, 2, 3, 4];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_depth_first_search() {
        let graph = get_graph_from_json();
        let result = graph.depth_first_search(0);

        // expected order of visited nodes: 0, 2, 3, 4, 1
        assert_eq!(result, vec![0, 2, 3, 4]);
    }

    #[test]
    fn add_node_test() {
        let mut graph = get_graph_from_json();
        graph.add_node(5, "F".to_string());
        let expected_node = Node {
            id: 5,
            label: "F".to_string(),
        };

        assert_eq!(graph.nodes[5], expected_node);
    }

    #[test]
    fn remove_node_test() {
        let mut graph = get_graph_from_json();
        graph.remove_node(4);
        assert!(graph.nodes.get(5).is_none());
    }

    #[test]
    fn add_edge_test() {
        let mut graph = get_graph_from_json();
        graph.add_edge(4, 1);

        let expected_edge = Edge {
            source: 4,
            target: 1,
        };

        assert_eq!(graph.edges[4], expected_edge);
    }

    #[test]
    fn remove_edge_test() {
        let mut graph = get_graph_from_json();
        graph.remove_edge(3, 4);
        assert!(graph.edges.get(3).is_none());
    }

    #[test]
    fn to_json_test(){
        let mut graph = get_graph_from_json();
        graph.add_node(5, "H".to_string());
        graph.add_edge(4, 5);
        graph.to_json("graph_test.json".to_string());

        let mut file = File::open("graph_test.json".to_string()).expect("Error during reading file");
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let graph = Graph::from_json(&data);

        let expected_nodes = Node {
            id: 5,
            label: "H".to_string()
        };

        let expected_edge = Edge {
            source: 4,
            target: 5,
        };

        assert_eq!(graph.nodes[5], expected_nodes);
        assert_eq!(graph.edges[4], expected_edge);
    }
}
