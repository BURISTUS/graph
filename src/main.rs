use graph::graph::Graph;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("graph.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut graph = Graph::from_json(&contents);
    println!("Graph from file {:#?}", graph);

    
    graph.remove_edge(3, 4);
    println!("Graph after removing edge with source 3, target 4 {:#?}", graph.edges);
    
    graph.remove_node(4);
    println!("Graph after removing node with id 4: {:#?}", graph.nodes);
    
    graph.add_node(4, "E".to_string());
    println!("Graph after adding node with id 4 and label E: {:#?}", graph.nodes);

    graph.add_edge(3, 4);
    println!("Graph after adding edge with source 3, target 4 {:#?}", graph.edges);
    
    let bfs_result = graph.breadth_first_search(0);
    println!("BFS result: {:?}", bfs_result);

    let dfs_result = graph.depth_first_search(0);
    println!("DFS result: {:?}", dfs_result);
    
    graph.to_json("graph.json".to_string());
}
