use std::collections::{HashSet, HashMap};

// A Node struct describes a step in the build process.
#[derive(Eq, PartialEq, Hash)]
struct Node {
    id : i32,
    dependencies: Vec<String>,
    targets: Vec<String>,
}
impl Node {
    fn process(&self) {
        println!("Node {} processed", self.id);
    }
}
struct Graph {
    nodes : Vec<Node>,
}
impl Graph {
    fn adj_nodes(&self, node : &Node) -> Vec<&Node> {
        let mut neighbors = Vec::new();
        for neighbor in &self.nodes {
            let dependencies = &neighbor.dependencies;
            let targets = &node.targets;
            let dep_set : HashSet<_> = dependencies.iter().collect();
            let targ_set : HashSet<_> = targets.iter().collect();
            let intersect : HashSet<_>;
            intersect = dep_set.intersection(&targ_set).collect();
            if intersect.len() > 0 {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
    fn process(&self) {
        let mut visited_nodes = HashSet::new();
        for node in &self.nodes {
            visited_nodes.insert(node);
        }
    }
    fn traverse<'a>(&'a self, root : &'a Node, 
                visited_nodes : &mut HashSet<&'a Node>) {
        if visited_nodes.contains(root) {
            return;
        }
        root.process();
        visited_nodes.insert(root);
        for neighbor in self.adj_nodes(root) {
            self.traverse(neighbor, visited_nodes);
        }
    }
}
pub fn test() {
    println!("butt hammer");
}
