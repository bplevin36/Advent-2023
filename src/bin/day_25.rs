use std::{time::Instant, collections::HashMap};

use petgraph::{dot::{Dot, Config}, prelude::*};
use rand::{seq::SliceRandom, thread_rng};
use aoc2023::read_input;

#[derive(Default, Clone)]
struct Graph {
    g: StableUnGraph<String, ()>,
}

impl Graph {
    fn get_or_add_node(&mut self, name: &str, nodes: &mut HashMap<String, NodeIndex>) -> NodeIndex {
        match nodes.get(name) {
            Some(&node_idx) => node_idx,
            None => {
                let node_idx = self.g.add_node(name.to_owned());
                nodes.insert(name.to_owned(), node_idx);
                node_idx
            },
        }
    }

    fn add_edge(&mut self, src: NodeIndex, dst: NodeIndex) -> EdgeIndex {
        let edge_idx = self.g.add_edge(src, dst, ());
        edge_idx
    }

    fn redirect_edge(&mut self, edge: EdgeIndex, node_to_remove: NodeIndex, new_node: NodeIndex) {
        let (node1, node2) = self.g.edge_endpoints(edge).unwrap();
        if node1 == node_to_remove && node2 != node_to_remove{
            self.g.add_edge(node2, new_node, ());
        } else if node2 == node_to_remove && node1 != node_to_remove {
            self.g.add_edge(node1, new_node, ());
        }
        self.g.remove_edge(edge);
    }
    fn check_self_edges(&self) {
        for edge in self.g.edge_indices() {
            let (n1, n2) = self.g.edge_endpoints(edge).unwrap();
            if n1 == n2 {
                let dot = Dot::with_config(&self.g, &[Config::EdgeNoLabel]);
                println!("{:?}", dot);
                panic!("{:?} has self-edge", n1);
            }
        }
    }
    fn contract_edge(&mut self, node1: NodeIndex, node2: NodeIndex) {
        if node1 == node2 {
            let dot = Dot::with_config(&self.g, &[Config::EdgeNoLabel]);
            println!("{:?}", dot);
            panic!("Cannot contract a self-edge {:?}", node1);
        }
        // remove all edges between the nodes we're contracting
        let edges: Vec<EdgeIndex> = self.g.edges_connecting(node1, node2).map(|r| r.id()).collect();
        for redundant_edge in edges {
            self.g.remove_edge(redundant_edge);
        }

        let node1_name = &self.g[node1];
        let node2_name = &self.g[node2];
        let mut new_node_name = node1_name.to_owned();
        new_node_name.push('_');
        new_node_name.push_str(node2_name);

        let new_node = self.g.add_node(new_node_name);
        // for the two old nodes, redirect all incoming edges to the new node
        for &node_to_remove in &[node1, node2] {
            let neighbors: Vec<NodeIndex> = self.g.neighbors(node_to_remove).collect();
            for neighbor in neighbors {
                let edge_out = self.g.find_edge(node_to_remove, neighbor).unwrap();
                self.redirect_edge(edge_out, node_to_remove, new_node);
            }
            // check that the node we're removing is no longer connected
            let neighbors: Vec<NodeIndex> = self.g.neighbors(node_to_remove).collect();
            if neighbors.len() > 0 {
                panic!("Node {:?} still has neighbors: {:?}", node_to_remove, neighbors);
            }
        }
        // remove both old nodes at the end to not invalidate any indices
        self.g.remove_node(node1);
        self.g.remove_node(node2);
        if let Some(self_edge) = self.g.find_edge(new_node, new_node) {
            panic!("Left a self-edge {:?}", self_edge);
        }
        self.check_self_edges();
    }

    fn contract_until_size(&mut self, size: usize) {
        let mut rng = thread_rng();
        while self.g.node_count() > size {
            let edges = self.g.edge_indices().collect::<Vec<EdgeIndex>>();
            let random_edge = edges.choose(&mut rng).unwrap();
            let (src_node, dst_node) = self.g.edge_endpoints(*random_edge).unwrap();
            self.contract_edge(src_node, dst_node);
        }
    }

    fn mincut(mut self) -> (Graph, usize) {
        if self.g.node_count() <= 6 {
            self.contract_until_size(2);
            let size = self.g.edge_count();
            (self, size)
        } else {
            let t = 1 + (self.g.node_count() as f32 / 2.0f32).ceil() as usize;
            let mut g1 = self.clone();
            let mut g2 = self.clone();
            g1.contract_until_size(t);
            g2.contract_until_size(t);
            let (g1, g1_size) = g1.mincut();
            let (g2, g2_size) = g2.mincut();
            if g1_size < g2_size {
                (g1, g1_size)
            } else {
                (g2, g2_size)
            }
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let input = read_input("25");

    let mut graph: Graph = Graph::default();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    for line in input.lines() {
        let mut fields = line.split(|c| c == ' ' || c == ':');
        let src_node = fields.next().unwrap();
        let src_node_idx = graph.get_or_add_node(src_node, &mut nodes);
        for dst_node in fields {
            if !dst_node.is_empty() {
                let dst_node_idx = graph.get_or_add_node(dst_node, &mut nodes);
                graph.add_edge(src_node_idx, dst_node_idx);
            }
        }
    }

    for _ in 0..100 {
        let (mingraph, size) = graph.clone().mincut();
        if size != 3 {
            continue;
        }
        let mut product = 1;
        let nodes: Vec<NodeIndex> = mingraph.g.node_indices().collect();
        if nodes.len() != 2 {
            continue;
        }
        for node in nodes {
            let label = mingraph.g.node_weight(node).unwrap().clone();
            let num_merged = label.split("_").count();
            product *= num_merged;
        }
        println!("{:?}", product);
        println!("Total time: {:?}", start_time.elapsed());
        return;
    }

}
