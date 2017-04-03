#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
use std::collections::{HashSet, HashMap};

// A Node struct describes a step in the build process.
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
struct Node {
    commands: Vec<String>,
    dependencies: Vec<String>,
    targets: Vec<String>,
}
impl Node {
    fn process(&self) {
        println!("{:?}", &self);
    }
}
#[derive(Serialize, Deserialize)]
struct Graph {
    nodes: Vec<Node>,
}
impl Graph {
    fn adj_nodes(&self, node: &Node) -> Vec<&Node> {
        let mut neighbors = Vec::new();
        for neighbor in &self.nodes {
            let dependencies = &neighbor.dependencies;
            let targets = &node.targets;
            let dep_set: HashSet<_> = dependencies.iter().collect();
            let targ_set: HashSet<_> = targets.iter().collect();
            let intersect: HashSet<_>;
            intersect = dep_set.intersection(&targ_set).collect();
            if intersect.len() > 0 {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
    fn has_prereq(&self, node: &Node) -> bool {
        for neighbor in &self.nodes {
            for targ in &neighbor.targets {
                for dep in &node.dependencies {
                    if targ == dep {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn process(&self) {
        let mut visited_nodes: HashSet<&Node> = HashSet::new();
        for node in &self.nodes {
            if !(self.has_prereq(node)) {
                self.traverse(node, &mut visited_nodes);
            }
        }
    }
    fn traverse<'a>(&'a self, root: &'a Node, visited_nodes: &mut HashSet<&'a Node>) {
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
    println!("Running test");
    let s: String = String::from("
nodes:
    - commands:
        - Executing Node 1
      dependencies:
        - B
      targets:
        - C
    - commands:
          - Executing Node 2
      dependencies:
        - A
      targets:
        - B
");
    let deserialized: Graph = serde_yaml::from_str(&s).unwrap();
    deserialized.process();
}
