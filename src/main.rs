use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

impl Step {
    fn new(position: Node, cost: Cost, history: Vec<Node>) -> Self {
        Step {
            cost,
            position,
            history,
        }
    }

    fn from_start(position: Node) -> Self {
        Step::new(position, 0, vec![])
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
            .then_with(|| other.history.cmp(&self.history))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut dist: HashMap<Node, Cost> = g.nodes.iter()
        .map(|&node| if node == start { (node, 0) } else { (node, usize::MAX) })
        .collect();

    let mut priority_queue = BinaryHeap::new();

    priority_queue.push(Step::from_start(start));

    while let Some(Step { cost, position, mut history}) = priority_queue.pop() {
        if position == goal {
            history.push(position);
            return Some((history, cost));
        }

        if let Some(neighbors) = &g.edges.get(&position) {
            for &(next_destination, edge_cost) in neighbors.iter() {
                let Some(candidate_cost) = cost.checked_add(edge_cost) else {
                    continue;
                };

                if candidate_cost < dist[&next_destination] {
                    dist.insert(next_destination, candidate_cost);

                    let mut next_history = history.clone();
                    next_history.push(position);

                    priority_queue.push(Step::new(next_destination, candidate_cost, next_history));
                }
            }
        }
    }

    None
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(
            &g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24); 
}
