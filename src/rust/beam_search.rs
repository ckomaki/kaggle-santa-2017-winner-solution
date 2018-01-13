use assignment::Assignment;
use input::Input;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use std::i32;
use std::i64;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use slot::Slot;
use score::Score;
use loop_solver::LoopSolver;
use std::collections::HashSet;


struct Util {
}

impl Util {
    pub fn get_uid(capacities: &Slot) -> String {
        let mut uid = 1 as i64;
        for r in 0..1_000 {
            if capacities.has(r) {
                uid = uid.wrapping_mul(1_000_000_009 as i64).wrapping_add(capacities.get_one(r) as i64);
                uid = uid.wrapping_mul(1_000_000_009 as i64).wrapping_add(capacities.get_two(r) as i64);
                uid = uid.wrapping_mul(1_000_000_009 as i64).wrapping_add(capacities.get_three(r) as i64);
            } else {
                uid = uid.wrapping_mul(1_000_000_009 as i64).wrapping_add(0 as i64);
                uid = uid.wrapping_mul(1_000_000_009 as i64).wrapping_add(0 as i64);
                uid = uid.wrapping_mul(1_000_000_009 as i64).wrapping_add(0 as i64);
            }
        }
        format!("{}", uid).to_string()
    }

    pub fn exist(base_dir: &str, uid: &str) -> bool {
        Path::new(&format!("{}/{}", base_dir, uid)).exists()
    }

    pub fn save_if_not_exist(
        base_dir: &str,
        assignment: &Assignment,
        base_assignment: &Assignment,
        input: &Input,
        capacities: &Slot,
    ) {
        let uid = Util::get_uid(&capacities);
        if !Util::exist(base_dir, &uid) {
            fs::remove_dir_all(&format!("{}/{}_tmp", base_dir, uid));

            fs::create_dir_all(&format!("{}/{}_tmp", base_dir, uid));
            assignment.save_diff(base_assignment, &format!("{}/{}_tmp/assignment", base_dir, uid));
            assignment.save_slotted(&format!("{}/{}_tmp/slotted", base_dir, uid));
            capacities.save(&format!("{}/{}_tmp/capacities", base_dir, uid));
            assignment.compute_score(input).save(&format!("{}/{}_tmp/scores", base_dir, uid));

            fs::rename(&format!("{}/{}_tmp", base_dir, uid),&format!("{}/{}", base_dir, uid));
        }
    }

    pub fn load_assignment(base_dir: &str, uid: &str, base_assignment: &Assignment) -> Assignment {
        Assignment::load_diff(&base_assignment, &format!("{}/{}/assignment", base_dir, uid))
    }

    pub fn load_slotted(base_dir: &str, uid: &str) -> Slot {
        Slot::load(&format!("{}/{}/slotted", base_dir, uid))
    }

    pub fn load_capacities(base_dir: &str, uid: &str) -> Slot {
        Slot::load(&format!("{}/{}/capacities", base_dir, uid))
    }

    pub fn load_score(base_dir: &str, uid: &str) -> Score {
        Score::load(&format!("{}/{}/scores", base_dir, uid))
    }
}







#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct QueueNode {
    lscore: i64,
    rscore: i64,
    uid: String,
}

pub struct Queues {
    queues: Vec<BinaryHeap<QueueNode>>,
    visited: HashSet<i64>,
}

impl Queues {
    pub fn new(base_dir: &str, base_assignment: &Assignment, input: &Input) -> Queues {
        let mut queues = vec![];
        for _ in 0..1001 {
            queues.push(BinaryHeap::<QueueNode>::new());
        }

        let mut empty_capacities = Slot::create_nones();
        queues[0].push(QueueNode {
            lscore: 0 as i64,
            rscore: 0,
            uid: Util::get_uid(&empty_capacities),
        });
        Util::save_if_not_exist(base_dir, base_assignment, base_assignment, input, &empty_capacities);

        Queues { queues, visited: HashSet::<i64>::new() }
    }

    pub fn pop(&mut self, depth: usize) -> Option<QueueNode> {
        self.queues[depth].pop()
    }

    pub fn is_visited(&self, uid: &str) -> bool {
        let uid_i64 = uid.parse::<i64>().unwrap();
        self.visited.contains(&uid_i64)
    }

    pub fn visit(&mut self, uid: &str) {
        let uid_i64 = uid.parse::<i64>().unwrap();
        self.visited.insert(uid_i64);
    }

    pub fn push(&mut self, depth: usize, uid: &str, score: &Score) {
        self.queues[depth].push(QueueNode {
            lscore: score.get_l_score(),
            rscore: score.get_r_score(),
            uid: uid.to_string(),
        })
    }

    pub fn size(&self, depth: usize) -> usize {
        self.queues[depth].len()
    }
}


struct LowerBound {
    l_score: i64,
    r_score: i64,
}

impl LowerBound {
    pub fn new() -> LowerBound {
        LowerBound { l_score: 0, r_score: 0 }
    }

    pub fn is_bigger(&self, score: &Score) -> bool {
        (self.l_score < score.get_l_score()) || ((self.l_score == score.get_l_score()) && (self.r_score < score.get_r_score()))
    }

    pub fn update(&mut self, score: &Score) {
        self.l_score = score.get_l_score();
        self.r_score = score.get_r_score();
    }
}



pub struct BeamSearch {
    pub base_assignment: Arc<Assignment>,
    pub base_dir: Arc<String>,
    pub input: Arc<Input>,
}

impl BeamSearch {

    pub fn solve_all(&self, candidates: &Vec<(Slot, String)>, thread_num: usize) {
        let mut tasks: Vec<(Slot, String)> = vec![];
        for candidate in candidates {
            if Util::exist(&self.base_dir, &Util::get_uid(&candidate.0)) {
                continue;
            }
            tasks.push(candidate.clone());
        }

        for begin in 0..tasks.len() {
            if begin % thread_num != 0 {
                continue;
            }
            println!("Running {}/{} jobs", begin, tasks.len());

            let mut handlers = vec![];
            for i in begin..begin + thread_num {
                if tasks.len() <= i {
                    continue;
                }

                let capacities = tasks[i].0.clone();
                let previous_uid = tasks[i].1.clone();
                let input = self.input.clone();
                let base_dir = self.base_dir.clone();
                let base_assignment = self.base_assignment.clone();

                handlers.push(thread::spawn(move || {
                    let previous_assignment = Util::load_assignment(&base_dir, &previous_uid, &base_assignment);
                    let capacity_assignment = previous_assignment.create_follow_capacity(&capacities, &input);
                    let next_assignment = LoopSolver::new(&input).optimize(&capacity_assignment, &capacities);
                    Util::save_if_not_exist(&base_dir, &next_assignment, &base_assignment, &input, &capacities);
                    let score = next_assignment.compute_score(&input);
                    println!("score: {}, {}, {}", score.get_full_score(), score.get_l_score(), score.get_r_score());
                }));
            }

            for handler in handlers {
                handler.join();
            }
        }
    }

    pub fn select_next_r(&self, uid: &str) -> Option<usize> {
        let capacities = Util::load_capacities(&self.base_dir, uid);
        let slotted = Util::load_slotted(&self.base_dir, uid);

        let important_rs = vec![118, 139, 191, 207, 320, 389, 671, 240, 204, 998, 555];
        for &r in &important_rs {
            if !capacities.has(r) {
                if slotted.get_two(r) % 2 != 0 || true {
                    println!("Important: {}", r);
                    return Some(r);
                }
            }
        }

        for r in 0..1_000 {
            if !capacities.has(r) {
                if slotted.get_three(r) % 3 != 0 && slotted.get_two(r) % 2 != 0 {
                    println!("2 & 3: {}", r);
                    return Some(r);
                }
            }
        }

        for r in 0..1_000 {
            if !capacities.has(r) {
                if slotted.get_three(r) % 3 != 0 {
                    println!("3: {}", r);
                    return Some(r);
                }
            }
        }

        for r in 0..1_000 {
            if !capacities.has(r) {
                if slotted.get_two(r) % 2 != 0 {
                    println!("2: {}", r);
                    return Some(r);
                }
            }
        }

        Option::None
    }

    pub fn search(&mut self) {
        let thread_num = 48;
        let mut limit_three = 15;
        let mut limit_two = 15;
        let mut limit_two_three = 11;


        let mut lower_bound = LowerBound::new();
        lower_bound.update(&Score { l_score: 11799596220, r_score: 7703388,});
        let mut queues = Queues::new(&self.base_dir, &self.base_assignment, &self.input);

        loop {
            for depth in 0..1000 {

                let mut candidates: Vec<(Slot, String)> = vec![];
                for _ in 0..24 {

                    if let Some(node) = queues.pop(depth) {
                        // Check score
                        let score = Util::load_score(&self.base_dir, &node.uid);
                        if 0 < depth && !lower_bound.is_bigger(&score) {
                            continue;
                        }

                        let next_r = self.select_next_r(&node.uid);
                        if next_r.is_none() {
                            let assignment = Util::load_assignment(&self.base_dir, &node.uid, &self.base_assignment);
                            println!("Update best score {}, {}, {}", score.get_full_score(), score.get_l_score(), score.get_r_score());
                            let score = assignment.compute_score(&self.input);
                            println!("Score: {}, {}, {}", score.get_full_score(), score.get_l_score(), score.get_r_score());
                            lower_bound.update(&score);
                            assignment.save_submission(&format!("/home/thinkpad_1/local/kaggle/santa_2017/submission"));
                            continue;
                        }
                        let next_r = next_r.unwrap();

                        let capacities = Util::load_capacities(&self.base_dir, &node.uid);
                        let slotted = Util::load_slotted(&self.base_dir, &node.uid);
                        for three in 0..1000 {
                            for two in 0..1000 {
                                let one = 1000 - three - two;
                                if one < 0 || three % 3 != 0 || two % 2 != 0 {
                                    continue;
                                }
                                if limit_three <= (slotted.get_three(next_r) as i32 - three as i32).abs() {
                                    continue;
                                }
                                if limit_two <= (slotted.get_two(next_r) as i32 - two as i32).abs() {
                                    continue;
                                }
                                if limit_two_three <= ((slotted.get_two(next_r) + slotted.get_three(next_r)) as i32 - ((three + two) as i32)).abs() {
                                    continue;
                                }

                                let mut next_capacities = capacities.clone();
                                next_capacities.set(next_r, one, two, three);

                                if !next_capacities.is_valid() {
                                    continue;
                                }

                                if queues.is_visited(&Util::get_uid(&next_capacities)) {
                                    println!("Visited");
                                    continue;
                                }
                                queues.visit(&Util::get_uid(&next_capacities));

                                candidates.push((next_capacities, node.uid.clone()));
                            }
                        }

                    }
                }

                if candidates.is_empty() {
                    continue;
                }

                println!("depth: {}, candidates: {}, queue size: {}", depth, candidates.len(), queues.size(depth));

                self.solve_all(&candidates, thread_num);

                for candidate in candidates {
                    let capacities = candidate.0;
                    let uid = Util::get_uid(&capacities);
                    assert!(Util::exist(&self.base_dir, &uid));

                    let score = Util::load_score(&self.base_dir, &uid);

                    if lower_bound.is_bigger(&score) {
                        println!("Add to queue: {}, {}, {}", score.get_full_score(), score.get_l_score(), score.get_r_score());
                        queues.push(depth + 1, &uid, &score);
                    } else {
                        println!("Skip: {}, {}, {}", score.get_full_score(), score.get_l_score(), score.get_r_score());
                    }
                }
            }

        }

    }
}



