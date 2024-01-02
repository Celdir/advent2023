#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use cgraph::algo::flow::dinic::dinic;
use cgraph::graph::builder::GraphBuilder;
use cgraph::graph::flow::FlowGraph;
use cgraph::graph::traits::{GraphIter, OrdinalGraph};
use cgraph::iter::bfs::bfs_where;

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut g = GraphBuilder::<(), isize>::new().adj_flat().flow().build();

    let mut ids: HashMap<&str, usize> = HashMap::new();

    for line in &input {
        let (node, tail) = line.split_once(": ").unwrap();
        if !ids.contains_key(&node) {
            let id = g.insert_node(());
            ids.insert(node, id);
        }
        for other in tail.split(" ") {
            if !ids.contains_key(&other) {
                let id = g.insert_node(());
                ids.insert(other, id);
            }
            g.insert_flow_edge(ids[node], ids[other], 1).unwrap();
            g.insert_flow_edge(ids[other], ids[node], 1).unwrap();
        }
    }

    let start = 0;
    let end = g.len().0 - 1;
    let flow = dinic(&mut g, start, end).unwrap();
    assert_eq!(flow, 3);
    let component_size = bfs_where(&g, start, |edge, _| edge.has_residual()).count();
    let rest_size = g.len().0 - component_size;
    let ans = component_size * rest_size;
    println!("{}", ans);
}
