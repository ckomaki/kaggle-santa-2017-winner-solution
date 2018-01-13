use std::fs::File;
use input::Input;
use std::io::{Write, BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::{From};
use std::collections::BinaryHeap;
use std::ops::{Neg, Add, Sub, Mul};
use std::io;
use std::cmp;
use std::fmt::Debug;
use std::cmp::Ordering;
use std::marker::Copy;
use slot::Slot;
use score::Score;


#[derive(Clone, Debug,)]
pub struct Assignment {
    pub slotted: Slot,
    pub assignments: Vec<usize>,
}

impl Assignment {

    pub fn new(assignments: Vec<usize>) -> Assignment {
        let slotted = Slot::create_from_assignment(&assignments);
        return Assignment { assignments, slotted }
    }

    pub fn create_follow_twin_triplet_constraint(&self, input: &Input) -> Assignment {
        let slotted = Slot::create_from_assignment(&self.assignments);

        for r in 0..1_000 {
            assert_eq!(0, slotted.get_three(r) % 3);
            assert_eq!(0, slotted.get_two(r) % 2);
            assert_eq!(0, slotted.get_one(r) % 1);
        }

        let mut assignments = vec![1_000; 1_000_000];
        let mut use_three = vec![0; 1000];
        let mut use_two = vec![0; 1000];
        let mut use_one = vec![0; 1000];
        for l in 0..5_001 {
            if (l % 3 == 0) && (self.assignments[l] == self.assignments[l + 1]) && (self.assignments[l] == self.assignments[l + 2]) {
                for i in 0..3 {
                    assignments[l + i] = self.assignments[l + i];
                    use_three[self.assignments[l + i]] += 1;
                }
            }
        }
        for l in 5_001..45_001 {
            if (l % 2 == 1) && (self.assignments[l] == self.assignments[l + 1]) {
                for i in 0..2 {
                    assignments[l + i] = self.assignments[l + i];
                    use_two[self.assignments[l + i]] += 1;
                }
            }
        }
        for l in 45_001..1_000_000 {
            assignments[l] = self.assignments[l];
            use_one[self.assignments[l]] += 1;
        }



        for l in 0..5_001 {
            let r = self.assignments[l];
            if assignments[l] != 1_000 {
                continue;
            }
            if input.get_lr_scores_12000(l, r) == (0, 0) {
                continue;
            }
            if use_three[r] == slotted.get_three(r) {
                continue;
            }

            assignments[l / 3 * 3 + 0] = r;
            assignments[l / 3 * 3 + 1] = r;
            assignments[l / 3 * 3 + 2] = r;
            use_three[r] += 3;
        }

        for range in vec![(35_000, 45_001), (5_001, 35_000)] {
            for l in range.0..range.1 {
                let r = self.assignments[l];
                if assignments[l] != 1_000 {
                    continue;
                }
                if input.get_lr_scores_12000(l, r) == (0, 0) {
                    continue;
                }
                if use_two[r] == slotted.get_two(r) {
                    continue;
                }

                let b = l + (l % 2);
                assignments[b - 0] = r;
                assignments[b - 1] = r;
                use_two[r] += 2;
            }
        }






        for l in 0..5_001 {
            if assignments[l] != 1_000 {
                continue;
            }
            for r in 0..1_000 {
                if use_three[r] < slotted.get_three(r) {
                    use_three[r] += 1;
                    assignments[l] = r;
                    break;
                }
            }
        }

        for l in 5_001..45_001 {
            if assignments[l] != 1_000 {
                continue;
            }
            for r in 0..1_000 {
                if use_two[r] < slotted.get_two(r) {
                    use_two[r] += 1;
                    assignments[l] = r;
                    break;
                }
            }
        }

        for l in 0..5_001 {
            if l % 3 == 0 {
                assert_eq!(assignments[l], assignments[l + 1]);
                assert_eq!(assignments[l], assignments[l + 2]);
            }
        }
        for l in 5_001..45_001 {
            if l % 2 == 1 {
                assert_eq!(assignments[l], assignments[l + 1]);
            }
        }
        for &r in &assignments {
            assert_ne!(1_000 as usize, r);
        }

        for l in 0..1_000_000 {
            if input.get_lr_scores_12000(l, assignments[l]) != input.get_lr_scores_12000(l, self.get_assignment(l)) {
                println!("{}, {}, {}", l, assignments[l], self.get_assignment(l));
                println!("{}, {:?}, {:?}", l, input.get_lr_scores_12000(l, assignments[l]), input.get_lr_scores_12000(l, self.get_assignment(l)));
            }
        }

        let assignment = Assignment::new(assignments);

        let score_before = self.compute_score(&input);
        let score_after = assignment.compute_score(&input);
        assert_eq!(score_before.get_l_score(), score_after.get_l_score());
        assert_eq!(score_before.get_r_score(), score_after.get_r_score());

        assignment
    }

    pub fn create_follow_capacity(&self, capacities: &Slot, input: &Input) -> Assignment {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct E {
            score: i64,
            l: usize,
        }

        let mut binary_heap = BinaryHeap::<E>::new();
        for l in 0..1_000_000 {
            binary_heap.push(E { l, score: input.get_l_1_000_000_plus_r(l, self.assignments[l]), } );
        }

        let mut cap_one = vec![0; 1_000];
        let mut cap_two = vec![0; 1_000];
        let mut cap_three = vec![0; 1_000];
        let mut cap_other = vec![0; 1_000];
        let mut cap_other_one = 1_000_000 - 45_001;
        let mut cap_other_two = 45_001 - 5_001;
        let mut cap_other_three = 5_001;

        for r in 0..1_000 {
            if capacities.has(r) {
                assert_eq!(1_000, capacities.get_one(r) + capacities.get_two(r) + capacities.get_three(r));
                cap_one[r] = capacities.get_one(r);
                cap_two[r] = capacities.get_two(r);
                cap_three[r] = capacities.get_three(r);

                cap_other_one -= capacities.get_one(r);
                cap_other_two -= capacities.get_two(r);
                cap_other_three -= capacities.get_three(r);
            } else {
                cap_other[r] = 1_000;
            }
        }

        let mut assignments = vec![1_000; 1_000_000];
        while let Some(e) = binary_heap.pop() {
            let r = self.assignments[e.l];
            if capacities.has(r) {
                if e.l < 5_001 {
                    if 0 < cap_three[r] {
                        cap_three[r] -= 1;
                        assignments[e.l] = r;
                    }
                } else if e.l < 45_001 {
                    if 0 < cap_two[r] {
                        cap_two[r] -= 1;
                        assignments[e.l] = r;
                    }
                } else {
                    if 0 < cap_one[r] {
                        cap_one[r] -= 1;
                        assignments[e.l] = r;
                    }
                }
            } else {
                if e.l < 5_001 {
                    if 0 < cap_other_three && 0 < cap_other[r] {
                        cap_other_three -= 1;
                        cap_other[r] -= 1;
                        assignments[e.l] = r;
                    }
                } else if e.l < 45_001 {
                    if 0 < cap_other_two && 0 < cap_other[r] {
                        cap_other_two -= 1;
                        cap_other[r] -= 1;
                        assignments[e.l] = r;
                    }
                } else {
                    if 0 < cap_other_one && 0 < cap_other[r] {
                        cap_other_one -= 1;
                        cap_other[r] -= 1;
                        assignments[e.l] = r;
                    }
                }
            }
        }

        for l in 0..1_000_000 {
            if assignments[l] != 1_000 {
                continue;
            }
            for r in 0..1_000 {
                if capacities.has(r) {
                    if l < 5_001 {
                        if 0 < cap_three[r] {
                            cap_three[r] -= 1;
                            assignments[l] = r;
                            break;
                        }
                    } else if l < 45_001 {
                        if 0 < cap_two[r] {
                            cap_two[r] -= 1;
                            assignments[l] = r;
                            break;
                        }
                    } else {
                        if 0 < cap_one[r] {
                            cap_one[r] -= 1;
                            assignments[l] = r;
                            break;
                        }
                    }
                } else {
                    if l < 5_001 {
                        if 0 < cap_other_three && 0 < cap_other[r] {
                            cap_other_three -= 1;
                            cap_other[r] -= 1;
                            assignments[l] = r;
                            break;
                        }
                    } else if l < 45_001 {
                        if 0 < cap_other_two && 0 < cap_other[r] {
                            cap_other_two -= 1;
                            cap_other[r] -= 1;
                            assignments[l] = r;
                            break;
                        }
                    } else {
                        if 0 < cap_other_one && 0 < cap_other[r] {
                            cap_other_one -= 1;
                            cap_other[r] -= 1;
                            assignments[l] = r;
                            break;
                        }
                    }
                }
            }
        }

        for &r in &assignments {
            assert_ne!(1_000 as usize, r);
        }

        Assignment::new(assignments)
    }

    pub fn clone_assignments(&self) -> Vec<usize> {
        self.assignments.clone()
    }

    pub fn get_assignment(&self, l: usize) -> usize {
        self.assignments[l]
    }

    pub fn compute_score(&self, input: &Input) -> Score {
        let mut l_score = 0;
        let mut r_score = 0;
        for l in 0..1_000_000 {
            let (ls, rs) = input.get_lr_scores_12000(l, self.assignments[l]);
            l_score += ls as i64;
            r_score += rs as i64;
        }

        Score::new(l_score, r_score)
    }

    pub fn load_submission(path: &str) -> Assignment {
        let f = BufReader::new(File::open(path).unwrap());
        let mut lines = vec![];
        for line in f.lines() {
            lines.push(line.unwrap());
        }
        assert_eq!(1_000_001, lines.len());

        let mut assignments: Vec<usize> = vec![];
        for line_i in 1..lines.len() {
            assignments.push(lines[line_i].clone().split(",").collect::<Vec<&str>>().get(1).unwrap().parse::<usize>().unwrap());
        }

        Assignment::new(assignments)
    }

    pub fn save_submission(&self, dir_path: &str) {
        let path = format!(
            "{}/submission_{}.txt",
            dir_path,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        );
        println!("writing submission: {}", path);
        let mut file = File::create(path).unwrap();
        file.write("ChildId,GiftId\n".as_bytes()).ok();
        for l in 0..1_000_000 {
            file.write(format!("{},{}\n", l, self.assignments[l]).as_bytes()).ok();
        }
    }

    pub fn load_slotted(path: &str) -> Slot {
        Slot::load(path)
    }

    pub fn save_slotted(&self, path: &str) {
        self.slotted.save(path);
    }

    pub fn load_diff(base: &Assignment, path: &str) -> Assignment {
        let f = BufReader::new(File::open(path).unwrap());
        let mut assignments = base.assignments.clone();
        for line in f.lines() {
            let diff: Vec<usize> = line.unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            assignments[diff[0]] = diff[1];
        }

        Assignment::new(assignments)
    }

    pub fn save_diff(&self, base: &Assignment, path: &str) {
        let mut file = File::create(path).unwrap();
        for l in 0..1_000_000 {
            if base.assignments[l] != self.assignments[l] {
                file.write(format!("{} {}\n", l, self.assignments[l]).as_bytes()).ok();
            }
        }
    }
}
