use std::i32;

pub struct LoopCanceler {
    distances: Vec<i64>,
    mid_nodes: Vec<usize>,
}

#[derive(Copy, Clone, Debug,)]
struct Node {
    distance: i64,
    previous_node: usize,
    cur_updated: bool,
    prev_updated: bool,
}

impl LoopCanceler {

    pub fn new() -> LoopCanceler {
        LoopCanceler {
            distances: vec![1_000_000_000_000_000 as i64; 3_000 * 3_000],
            mid_nodes: vec![1_000_000 as usize; 3_000 * 3_000],
        }
    }

    pub fn init(&mut self) {
        for i in 0..3_000 * 3_000 {
            self.distances[i] = 1_000_000_000_000_000 as i64;
            self.mid_nodes[i] = 1_000_000 as usize;
        }
    }

    pub fn get_index(source: usize, sink: usize) -> usize {
        source * 3_000 + sink
    }

    pub fn add_edge(&mut self, index: usize, mid: usize, distance: i64) {
        if distance < self.distances[index] {
            self.distances[index] = distance;
            self.mid_nodes[index] = mid;
        }
    }

    pub fn get_mid_node(&self, source: usize, sink: usize) -> usize {
        self.mid_nodes[LoopCanceler::get_index(source, sink)]
    }

    pub fn find_loops(&mut self) -> Vec<Vec<usize>> {
        let mut loops = vec![];
        for _ in 0..20 {
            if let Some(nodes) = self.find_one_loop() {
                for &node in &nodes {
                    for i in 0..3000 {
                        self.distances[LoopCanceler::get_index(i, node)] = 1_000_000_000_000_000 as i64;
                        self.distances[LoopCanceler::get_index(node, i)] = 1_000_000_000_000_000 as i64;
                    }
                }
                loops.push(nodes);
            } else {
                return loops;
            }
        }

        loops
    }

    pub fn find_one_loop(&mut self) -> Option<Vec<usize>> {
        let mut nodes: Vec<Node> = vec![];
        for _ in 0..3_000 {
            nodes.push(Node {
                distance: 0,
                previous_node: 0,
                cur_updated: true,
                prev_updated: true,
            });
        }

        let mut last_updated_node: usize = 3_000;
        for step in 0..3_050 {
            for i in 0..3_000 {
                nodes[i].prev_updated = nodes[i].cur_updated;
                nodes[i].cur_updated = false;
            }

            for source in 0..3000 {
                if !nodes[source].prev_updated {
                    continue;
                }

                let source_d = nodes[source].distance;

                let edge_index = (source * 3_000) as usize;
                for sink in 0..3_000 {
                    let sink_node = &mut nodes[sink];
                    let d = source_d + self.distances[edge_index + sink];
                    if d < sink_node.distance {
                        sink_node.distance = d;
                        sink_node.previous_node = source;
                        sink_node.cur_updated = true;
                    }
                }
            }

            last_updated_node = 3_000;
            for i in 0..3_000 {
                if !nodes[i].cur_updated {
                    continue;
                }
                last_updated_node = i;
                break;
            }

            if last_updated_node == 3_000 {
                return Option::None;
            }

            if 2 <= step && (step as i32).count_ones() == 1 {
                let mut updated_nodes = vec![];
                for i in 0..3_000 {
                    if !nodes[i].cur_updated {
                        continue;
                    }
                    updated_nodes.push(i);
                }

                for root_node in updated_nodes {
                    let mut root_node = root_node;
                    for _ in 0..3_000 {
                        root_node = nodes[root_node].previous_node;
                    }

                    let mut total_distance: i64 = 0;
                    let mut node = root_node;
                    let mut counter = 0;
                    while counter < 3050 {
                        let previous_node = nodes[node].previous_node;
                        total_distance += self.distances[LoopCanceler::get_index(previous_node, node)];
                        node = previous_node;
                        if node == root_node {
                            break;
                        }
                        counter += 1;
                    }

                    if counter < 3050 && total_distance < 0 {
                        let mut loop_nodes = vec![root_node];
                        let mut node = nodes[root_node].previous_node;
                        while node != root_node {
                            loop_nodes.push(node);
                            node = nodes[node].previous_node;
                        }
                        return Some(loop_nodes.iter().rev().map(|&v| v).collect());
                    }
                }
            }
        }

        let mut root_node = last_updated_node;
        for _ in 0..3050 {
            root_node = nodes[root_node].previous_node;
        }

        let mut loop_nodes = vec![root_node];
        let mut node = nodes[root_node].previous_node;
        while node != root_node {
            loop_nodes.push(node);
            node = nodes[node].previous_node;
        }

        return Some(loop_nodes.iter().rev().map(|v| *v).collect());
    }
}
