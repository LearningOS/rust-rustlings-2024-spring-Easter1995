/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

// 无向图结构的邻接表，用HashMap来存储，键是String，键值是一个向量，其中i32可能表示边长
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    // 获取邻接表的可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    // 获取邻接表的引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    // 添加边
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 要在三元组的第一个str和第二个str之间加边
        let (vertice1, vertice2, weight) = edge;
        
        // 如果第一个点查询得到
        if let Some(adj1) = self.adjacency_table_mutable().get_mut(&String::from(vertice1)) {
            adj1.push((String::from(vertice2), weight));
        } else {
            self.add_node(vertice1);
            if let Some(new_adj1) = self.adjacency_table_mutable().get_mut(vertice1) {
                new_adj1.push((String::from(vertice2), weight));
            }
        }

        // 如果第二个点查询得到
        if let Some(adj2) = self.adjacency_table_mutable().get_mut(vertice2) {
            adj2.push((String::from(vertice1), weight));
        } else {
            self.add_node(vertice2);
            if let Some(new_adj2) = self.adjacency_table_mutable().get_mut(&String::from(vertice2)) {
                new_adj2.push((String::from(vertice1), weight));
            }
        }
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    
    // 添加成功返回true，添加失败返回false
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            return false;
        }
        else {
            self.adjacency_table_mutable().insert(node.to_string(), Vec::new());
            return true;
        }
    }
    // 在结构体方法中实现了
    fn add_edge(&mut self, edge: (&str, &str, i32));
    
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}