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


pub struct Score {
    pub l_score: i64,
    pub r_score: i64,
}

impl Score {
    pub fn new(l_score: i64, r_score: i64) -> Score {
        Score { l_score, r_score }
    }

    pub fn get_l_score(&self) -> i64 {
        self.l_score
    }

    pub fn get_r_score(&self) -> i64 {
        self.r_score
    }

    pub fn get_full_score(&self) -> f64 {
        let mut l_score = self.l_score as f64;
        let mut r_score = self.r_score as f64;

        l_score /= 1_000_000.0;
        r_score /= 1_000_000.0;

        l_score = l_score * (2.0 * 100.0) / 12_000.0;
        r_score = r_score * (2.0 * 1_000.0) / 12_000.0;

        l_score -= 1.0;
        r_score -= 1.0;

        l_score /= 2.0 * 100.0;
        r_score /= 2.0 * 1_000.0;

        l_score * l_score * l_score + r_score * r_score * r_score
    }

    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write(format!("{} {}\n", self.l_score, self.r_score).as_bytes()).ok();
    }

    pub fn load(path: &str) -> Score {
        let mut f = BufReader::new(File::open(path).unwrap());
        let mut line = String::new();
        f.read_line(&mut line);
        let scores: Vec<i64> = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();

        Score {
            l_score: scores[0],
            r_score: scores[1],
        }
    }
}
