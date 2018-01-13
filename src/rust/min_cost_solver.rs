use data_structure::min_cost_flow_dup_binary_heap::MinCostFlowDupBinaryHeap;
use data_structure::min_cost_flow::MinCostFlow;
use input::Input;

pub struct MinCostSolver {
}

impl MinCostSolver {
    pub fn solve(input: &Input) {
        let mut min_cost_flow = MinCostFlowDupBinaryHeap::new(1000 + 1_000_000 + 3);

        let l_weight = 500;
        let source = 1000 + 1_000_000;
        let sink = source + 1;
        let dummy_r = sink + 1;
        let worst = 2_000_000_000 as i64;

        for l in 0..1_000_000 {
            min_cost_flow.add_edge(source, l, 1, 0);
            min_cost_flow.add_edge(l, dummy_r, 1, worst);

            for e in input.get_edges_lr(l) {
                assert!(0 < e.lscore + e.rscore);
                let cost = worst - (e.lscore * l_weight + e.rscore) as i64;
                assert!(0 < cost);
                min_cost_flow.add_edge(l, 1_000_000 + e.sink, 1, cost);
            }
        }
        for r in 0..1_000 {
            min_cost_flow.add_edge(1_000_000 + r, sink, 1000, 0);
        }
        min_cost_flow.add_edge(dummy_r, sink, 1_000_000, 0);

        let cost = min_cost_flow.flow(source, sink, 1_000_000, 1_000_000_000_000_000 as i64);
        println!("{}", cost);
        min_cost_flow.write_assignment();
    }
}

