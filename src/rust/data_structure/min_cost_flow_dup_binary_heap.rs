use data_structure::min_cost_flow::{MinCostFlow, I};

use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::{From};
use std::collections::BinaryHeap;
use std::ops::{Neg, Add, Sub, Mul};
use std::io;
use std::cmp;
use std::fmt::Debug;
use std::cmp::Ordering;
use std::marker::Copy;
use std::fs::File;
use std::io::Write;

struct Edge<CAPACITY, COST> {
    source: usize,
    dest: usize,
    reversed: usize,
    capacity: CAPACITY,
    cost: COST,
}

impl<CAPACITY: I, COST> Edge<CAPACITY, COST> {
    fn add_capacity(&mut self, v: CAPACITY) -> () {
        self.capacity = self.capacity + v;
    }
}

#[derive(PartialEq, Eq)]
struct QueueElement<COST: I> {
    distance: COST,
    node_index: usize,
}

impl<COST: I> PartialOrd for QueueElement<COST> {
    fn partial_cmp(&self, other: &QueueElement<COST>) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl<COST: I> Ord for QueueElement<COST> {
    fn cmp(&self, other: &QueueElement<COST>) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

#[derive(Default)]
struct TraversedEdge {
    node_index: usize,
    edge_index: usize,
}

#[derive(Default)]
struct Node<COST> {
    distance: COST,
    potential: COST,
    visited: bool,
    traversed_edges: Vec<TraversedEdge>,
}

pub struct MinCostFlowDupBinaryHeap<CAPACITY: I, COST: I> {
    edges: Vec<Vec<Edge<CAPACITY, COST>>>,
    nodes: Vec<Node<COST>>,
}

impl<CAPACITY: I, COST: I> MinCostFlowDupBinaryHeap<CAPACITY, COST> {
    pub fn new(node_num: usize) -> MinCostFlowDupBinaryHeap<CAPACITY, COST> {
        let mut edges: Vec<Vec<Edge<CAPACITY, COST>>> = vec![];
        let mut nodes: Vec<Node<COST>> = vec![];
        for _ in 0..node_num {
            edges.push(vec![]);
            nodes.push(Default::default());
        }

        MinCostFlowDupBinaryHeap { edges: edges, nodes: nodes }
    }

    fn update_distance(&mut self, flow_source: usize, flow_sink: usize, inf: COST, q: &mut BinaryHeap<QueueElement<COST>>) -> COST {
        for node in &mut self.nodes {
            node.distance = inf;
            node.traversed_edges.clear();
        }

        self.nodes[flow_source].distance = Default::default();
        q.clear();
        q.push(QueueElement{ node_index: flow_source, distance: Default::default()});

        while let Some(elem) = q.pop() {
            let node_index = elem.node_index;
            if self.nodes[flow_sink].distance < self.nodes[node_index].distance {
                break;
            }

            if self.nodes[node_index].distance < elem.distance {
                continue;
            }

            let dist_plus_potential = self.nodes[node_index].distance + self.nodes[node_index].potential;
            let edges = &self.edges[node_index];
            for edge_index in 0..edges.len() {
                let edge = &edges[edge_index];
                if edge.capacity == CAPACITY::default() {
                    continue;
                }

                let node: &mut Node<COST> = &mut self.nodes[edge.dest];
                let next_distance = dist_plus_potential - node.potential + edge.cost;

                if next_distance <= node.distance {
                    if next_distance < node.distance {
                        q.push(QueueElement{
                            node_index: edge.dest,
                            distance: next_distance,
                        });
                        node.distance = next_distance;
                        node.traversed_edges.clear();
                    }
                    node.traversed_edges.push(TraversedEdge {
                        node_index: node_index,
                        edge_index: edge_index,
                    });
                }
            }
        }

        self.nodes[flow_sink].distance
    }

    fn backtrace(&mut self, node_index: usize, source: usize, flow: CAPACITY) -> CAPACITY {
        if node_index == source {
            return flow;
        }
        self.nodes[node_index].visited = true;
        while !self.nodes[node_index].traversed_edges.is_empty() {
            let trav_edge = self.nodes[node_index].traversed_edges.pop().unwrap();
            if self.nodes[trav_edge.node_index].visited {
                continue;
            }

            let capacity = self.edges[trav_edge.node_index][trav_edge.edge_index].capacity;
            assert_ne!(CAPACITY::default(), capacity);

            let flowed = self.backtrace(trav_edge.node_index, source, cmp::min(capacity, flow));
            if flowed == CAPACITY::default() {
                continue;
            }

            self.nodes[node_index].visited = false;
            self.edges[trav_edge.node_index][trav_edge.edge_index].capacity -= flowed;
            let reversed_index = self.edges[trav_edge.node_index][trav_edge.edge_index].reversed;
            self.edges[node_index][reversed_index].capacity += flowed;
            if flowed < capacity {
                self.nodes[node_index].traversed_edges.push(trav_edge);
            }
            return flowed;
        }

        CAPACITY::default()
    }
}

impl<CAPACITY: I, COST: I + From<CAPACITY>> MinCostFlow<CAPACITY, COST> for MinCostFlowDupBinaryHeap<CAPACITY, COST> {
    fn add_edge(&mut self, source: usize, dest: usize, capacity: CAPACITY, cost: COST) -> &mut MinCostFlow<CAPACITY, COST> {
        let edge = Edge {
            source: source, dest: dest, capacity: capacity, cost: cost,
            reversed: self.edges[dest].len(),
        };
        let rev_edge = Edge {
            source: dest,
            dest: source,
            capacity: Default::default(),
            cost: -cost,
            reversed: self.edges[source].len(),
        };
        self.edges[source].push(edge);
        self.edges[dest].push(rev_edge);

        self
    }

    fn flow(&mut self, flow_source: usize, flow_sink: usize, flow: CAPACITY, inf: COST) -> COST {
        let mut q = BinaryHeap::<QueueElement<COST>>::new();
        let mut flow = flow;
        let mut total_cost: COST = Default::default();
        for node in &mut self.nodes {
            node.potential = Default::default();
        }

        while CAPACITY::default() < flow {
            println!("{}, {}", flow, total_cost);

            let sink_distance = self.update_distance(flow_source, flow_sink, inf, &mut q);
            if sink_distance == inf {
                return inf;
            }
            for node in &mut self.nodes {
                if node.distance < inf {
                    node.potential = node.potential + node.distance;
                }
                node.visited = false;
            }

            loop {
                let flowed = self.backtrace(flow_sink, flow_source, flow);
                if flowed == CAPACITY::default() {
                    break;
                }
                flow -= flowed;
                total_cost = total_cost + COST::from(flowed) * self.nodes[flow_sink].potential;
            }
        }

        total_cost
    }

    fn write_assignment(&self) {
        let s = format!("dup_binary_{}.txt", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        println!("{}", s);
        let mut file = File::create(s).unwrap();
        for l in 0..1_000_000 {
            let mut assignments = Vec::<usize>::new();
            for edge in &self.edges[l] {
                if 1_000_000 <= edge.dest && edge.dest < 1_000_000 + 1_000 {
                    if edge.capacity == CAPACITY::default() {
                        assignments.push(edge.dest);
                    }
                }
            }
            if assignments.is_empty() {
                assignments.push(1000);
            }
            file.write(format!("{} {}\n", l, assignments[0]).as_bytes());
        }
    }
}
