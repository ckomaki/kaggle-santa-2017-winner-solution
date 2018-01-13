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

#[derive(Clone, Debug,)]
struct CellSlot {
    one: usize,
    two: usize,
    three: usize,
}

#[derive(Clone, Debug,)]
pub struct Slot {
    cell_slots: Vec<Option<CellSlot>>,
}

impl Slot {
    pub fn create_nones() -> Slot {
        let mut cell_slots = vec![];
        for _ in 0..1_000 {
            cell_slots.push(Option::None);
        }

        Slot { cell_slots }
    }

    pub fn create_from_assignment(assignments: &Vec<usize>) -> Slot {
        let mut count_one = vec![0; 1_001];
        let mut count_two = vec![0; 1_001];
        let mut count_three = vec![0; 1_001];

        for l in 0..5_001 {
            count_three[assignments[l]] += 1;
        }
        for l in 5_001..45_001 {
            count_two[assignments[l]] += 1;
        }
        for l in 45_001..1_000_000 {
            count_one[assignments[l]] += 1;
        }

        let mut cell_slots = vec![];
        for r in 0..1_000 {
            cell_slots.push(Option::Some(CellSlot {
                one: count_one[r],
                two: count_two[r],
                three: count_three[r],
            }));
        }

        Slot { cell_slots }
    }

    pub fn has(&self, r: usize) -> bool {
        self.cell_slots[r].is_some()
    }

    pub fn get_one(&self, r: usize) -> usize {
        self.cell_slots[r].as_ref().unwrap().one
    }

    pub fn get_two(&self, r: usize) -> usize {
        self.cell_slots[r].as_ref().unwrap().two
    }

    pub fn get_three(&self, r: usize) -> usize {
        self.cell_slots[r].as_ref().unwrap().three
    }

    pub fn set(&mut self, r: usize, one: usize, two: usize, three: usize) {
        self.cell_slots[r] = Some(CellSlot { one, two, three });
    }

    pub fn is_valid(&self) -> bool {
        let mut one_num = 0;
        let mut two_num = 0;
        let mut three_num = 0;
        for slot in &self.cell_slots {
            if let Some(slot) = slot.as_ref() {
                assert_eq!(1_000, slot.one + slot.two + slot.three);
                one_num += slot.one;
                two_num += slot.two;
                three_num += slot.three;
            }
        }

        if 5_001 - 0 < three_num {
            return false;
        }
        if 45_001 - 5_001 < two_num {
            return false;
        }
        if 1_000_000 - 45_001 < one_num {
            return false;
        }

        true
    }

    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        for r in 0..1_000 {
            if let Some(slot) = self.cell_slots[r].as_ref() {
                file.write(format!("{} {} {} {}\n", r, slot.one, slot.two, slot.three).as_bytes()).ok();
            }
        }
    }

    pub fn load(path: &str) -> Slot {
        let f = BufReader::new(File::open(path).unwrap());
        let mut slot = Slot::create_nones();

        for line in f.lines() {
            let v: Vec<usize> = line.unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            assert_eq!(4, v.len());
            slot.set(v[0], v[1], v[2], v[3]);
        }
        return slot;
    }
}

