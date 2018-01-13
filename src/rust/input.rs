use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};


pub struct ScoreEdge {
    pub sink: usize,
    pub lscore: i32,
    pub rscore: i32,
}

impl ScoreEdge {
    pub fn get_lscore(&self) -> i32 {
        self.lscore
    }

    pub fn get_rscore(&self) -> i32 {
        self.rscore
    }

    pub fn get_sink(&self) -> usize {
        self.sink
    }
}

pub struct ScoreEdgeL1000000R {
    pub sink: usize,
    pub score: i64,
}


pub struct Input {
    edges_lr: Vec<Vec<ScoreEdge>>,
    scores_lr: Vec<HashMap<usize, (i32, i32)>>,

    l_1_000_000_plus_r: Vec<Vec<i64>>,
    l_1_000_000_plus_r_edges: Vec<Vec<ScoreEdgeL1000000R>>,
}

impl Input {
    pub fn get_l_1_000_000_plus_r(&self, l: usize, r: usize) -> i64 {
        self.l_1_000_000_plus_r[l][r]
    }

    pub fn get_l_1_000_000_plus_r_edges(&self, l: usize) -> &Vec<ScoreEdgeL1000000R> {
        &self.l_1_000_000_plus_r_edges[l]
    }

    pub fn get_lr_scores_12000(&self, l: usize, r: usize) -> (i32, i32) {
        *self.scores_lr[l].get(&r).unwrap_or(&(0, 0))
    }

    pub fn get_edges_lr(&self, l: usize) -> &Vec<ScoreEdge> {
        &self.edges_lr[l]
    }

    fn read_file(path: &str) -> Vec<Vec<usize>> {
        let mut preferences: Vec<Vec<usize>> = Vec::new();

        let f = File::open(path).expect(path);
        let f = BufReader::new(f);
        for line in f.lines() {
            let line = line.unwrap();
            let values: Vec<&str> = line.split(",").collect();
            let values: Vec<usize> = values.iter().map(|s| s.parse::<usize>().unwrap()).collect();
            preferences.push(values[1..].to_vec());
        }

        preferences
    }

    pub fn new_from_file(dir: &str) -> Input {
        let lr = Input::read_file(&[dir, "child_wishlist_v2.csv"].join("/"));
        let rl = Input::read_file(&[dir, "gift_goodkids_v2.csv"].join("/"));

        let mut edges_lr: Vec<Vec<ScoreEdge>> = vec![];
        for l in 0..1_000_000 {
            let mut edges: Vec<ScoreEdge> = vec![];
            edges_lr.push(edges);
        }

        let use_trick = true;
        println!("read input edge_lr");
        for l in 0..1_000_000 {
            for i in 0..100 {
                let r = lr[l][i];
                let preference = 12_000 * (2 * (100 - i as i32) + 1) / (2 * 100);

                if use_trick {
                    if l < 5_001 {
                        let b = l - l % 3;
                        edges_lr[b + 0].push(ScoreEdge { sink: r, lscore: preference / 3, rscore: 0 });
                        edges_lr[b + 1].push(ScoreEdge { sink: r, lscore: preference / 3, rscore: 0 });
                        edges_lr[b + 2].push(ScoreEdge { sink: r, lscore: preference / 3, rscore: 0 });
                    } else if l < 45_001 {
                        let b = if l % 2 == 0 { l - 1 } else { l };
                        edges_lr[b + 0].push(ScoreEdge { sink: r, lscore: preference / 2, rscore: 0 });
                        edges_lr[b + 1].push(ScoreEdge { sink: r, lscore: preference / 2, rscore: 0 });
                    } else {
                        edges_lr[l + 0].push(ScoreEdge { sink: r, lscore: preference / 1, rscore: 0 });
                    }
                } else {
                    edges_lr[l].push(ScoreEdge { sink: r, lscore: preference, rscore: 0 });
                }
            }
        }

        println!("read input edge_rl");
        for r in 0..1_000 {
            for i in 0..1_000 {
                let l = rl[r][i];
                let preference = 12_000 * (2 * (1_000 - i as i32) + 1) / (2 * 1_000);

                if use_trick {
                    if l < 5_001 {
                        let b = l - l % 3;
                        edges_lr[b + 0].push(ScoreEdge { sink: r, lscore: 0, rscore: preference / 3 });
                        edges_lr[b + 1].push(ScoreEdge { sink: r, lscore: 0, rscore: preference / 3 });
                        edges_lr[b + 2].push(ScoreEdge { sink: r, lscore: 0, rscore: preference / 3 });
                    } else if l < 45_001 {
                        let b = if l % 2 == 0 { l - 1 } else { l };
                        edges_lr[b + 0].push(ScoreEdge { sink: r, lscore: 0, rscore: preference / 2 });
                        edges_lr[b + 1].push(ScoreEdge { sink: r, lscore: 0, rscore: preference / 2 });
                    } else {
                        edges_lr[l + 0].push(ScoreEdge { sink: r, lscore: 0, rscore: preference / 1 });
                    }
                } else {
                    edges_lr[l].push(ScoreEdge { sink: r, lscore: 0, rscore: preference });
                }
            }
        }


        println!("read input compress");
        let mut lscores = vec![0; 1000];
        let mut rscores = vec![0; 1000];
        for l in 0..1_000_000 {
            for r in 0..1000 {
                lscores[r] = 0;
                rscores[r] = 0;
            }
            for e in &edges_lr[l] {
                lscores[e.sink] += e.lscore;
                rscores[e.sink] += e.rscore;
            }

            edges_lr[l] = vec![];
            for r in 0..1000 {
                if lscores[r] != 0 || rscores[r] != 0 {
                    edges_lr[l].push(ScoreEdge{
                        sink: r,
                        lscore: lscores[r],
                        rscore: rscores[r],
                    });
                }
            }
        }

        let mut scores_lr: Vec<HashMap<usize, (i32, i32)>> = vec![];
        for l in 0..1_000_000 {
            let mut score_map = HashMap::<usize, (i32, i32)>::new();
            for e in &edges_lr[l] {
                score_map.insert(e.sink, (e.lscore, e.rscore));
            }
            scores_lr.push(score_map);
        }

        let mut l_1_000_000_plus_r: Vec<Vec<i64>> = vec![];
        let mut l_1_000_000_plus_r_edges: Vec<Vec<ScoreEdgeL1000000R>> = vec![];
        for l in 0..1_000_000 {
            let mut v0 = vec![0; 1_000];
            let mut v1 = vec![];
            for e in &edges_lr[l] {
                let score = e.lscore as i64 * 1_000_000 as i64 + e.rscore as i64;
                v0[e.sink] = score;
                v1.push(ScoreEdgeL1000000R{
                    sink: e.sink,
                    score: score,
                });
            }
            l_1_000_000_plus_r.push(v0);
            l_1_000_000_plus_r_edges.push(v1);
        }


        println!("read input finish");
        Input {
            edges_lr,
            scores_lr,
            l_1_000_000_plus_r,
            l_1_000_000_plus_r_edges,
        }
    }
}
