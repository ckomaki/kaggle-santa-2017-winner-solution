use assignment::Assignment;
use std::collections::HashMap;
use loop_canceler::LoopCanceler;
use input::Input;
use std::time::{SystemTime, UNIX_EPOCH};
use slot::Slot;

pub struct LoopSolver<'a> {
    input: &'a Input,
}

impl<'a> LoopSolver<'a> {

    pub fn new(input: &Input) -> LoopSolver {
        LoopSolver { input }
    }

    pub fn convert_r(&self, l: usize, r_1000: usize, capacities: &Slot) -> usize {
        if !capacities.has(r_1000) {
            return r_1000 * 3;
        }
        if l < 5_001 {
            r_1000 * 3 + 0
        } else if l < 45_001 {
            r_1000 * 3 + 1
        } else {
            r_1000 * 3 + 2
        }
    }

    pub fn optimize(&self, assignment: &Assignment, capacities: &Slot) -> Assignment {
        let mut assignment = assignment.clone();
        let mut loop_canceler = LoopCanceler::new();

        loop {
            loop_canceler.init();

            for l in 0..1_000_000 {
                let r_1000 = assignment.get_assignment(l);
                let cur_gain = self.input.get_l_1_000_000_plus_r(l, r_1000);
                let r_3000 = self.convert_r(l, r_1000, capacities);
                for edge in self.input.get_l_1_000_000_plus_r_edges(l) {
                    let sink_r_3000 = self.convert_r(l, edge.sink, capacities);
                    loop_canceler.add_edge(
                        LoopCanceler::get_index(r_3000, sink_r_3000),
                        l,
                        cur_gain - edge.score
                    )
                }
            }

            for range in vec![
                (0..5_001).into_iter(),
                (5_001..45_001).into_iter(),
                (45_001..1_000_000).into_iter()
            ] {
                let inf = 1_000_000_000_000 as i64;
                let mut bests = vec![];
                for r in 0..1000 {
                    bests.push((inf, 0));
                }

                for l in range {
                    let r_1000 = assignment.get_assignment(l);
                    let cur_gain = self.input.get_l_1_000_000_plus_r(l, r_1000);
                    if cur_gain < bests[r_1000].0 {
                        bests[r_1000] = (cur_gain, l);
                    }
                }

                for source_r_1000 in 0..1000 {
                    let (cur_gain, l) = bests[source_r_1000];
                    if cur_gain == inf {
                        continue;
                    }

                    let source_r_3000 = self.convert_r(l, source_r_1000, capacities);
                    for sink_r_1000 in 0..1000 {
                        let sink_r_3000 = self.convert_r(l, sink_r_1000, capacities);
                        loop_canceler.add_edge(
                            LoopCanceler::get_index(source_r_3000, sink_r_3000),
                            l,
                            cur_gain - 0,
                        );
                    }
                }
            }

            let loops = loop_canceler.find_loops();
            if loops.is_empty() {
                return assignment;
            }

            let mut assignments = assignment.clone_assignments();
            for nodes in loops {
                let mut mid_nodes: Vec<usize> = vec![];
                for i in 0..nodes.len() {
                    let mid_node = loop_canceler.get_mid_node(nodes[i], nodes[(i + 1) % nodes.len()]);
                    mid_nodes.push(mid_node);
                    assert_eq!(assignment.get_assignment(mid_node), nodes[i] / 3);
                }
                for i in 0..nodes.len() {
                    assignments[mid_nodes[i]] = nodes[(i + 1) % nodes.len()] / 3;
                }
            }
            assignment = Assignment::new(assignments);
        }
    }

}


