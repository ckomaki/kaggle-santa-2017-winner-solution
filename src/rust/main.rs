#[allow(dead_code, unused_imports)]
mod assignment;
mod input;
mod loop_canceler;
mod min_cost_solver;
mod data_structure;
mod slot;
mod score;
mod loop_solver;
mod beam_search;

use std::process;
use assignment::Assignment;
use beam_search::BeamSearch;
use input::Input;
use loop_canceler::LoopCanceler;
use loop_solver::LoopSolver;
use min_cost_solver::MinCostSolver;
use std::sync::Arc;
use score::Score;
use slot::Slot;

fn create_submission(filename: &str, input: &Input) {
    let dir_path = "/home/thinkpad_1/local/kaggle/santa_2017/submission";
    let base_assignment = Assignment::load_submission(&format!("{}/{}", dir_path, filename));
    println!("base score: {}", base_assignment.compute_score(&input).get_l_score());
    println!("base score: {}", base_assignment.compute_score(&input).get_r_score());
    println!("base score: {}", base_assignment.compute_score(&input).get_full_score());

    let assignment = base_assignment.create_follow_twin_triplet_constraint(&input);
    assignment.save_submission(&dir_path);
    println!("base score: {}", assignment.compute_score(&input).get_l_score());
    println!("base score: {}", assignment.compute_score(&input).get_r_score());
    println!("base score: {}", assignment.compute_score(&input).get_full_score());

    process::exit(0);
}


fn main() {
    let input = Input::new_from_file("/home/thinkpad_1/local/kaggle/santa_2017/download");

    // create_submission("submission_1514218483.txt", &input);

    let base_assignment = Assignment::load_submission("/home/thinkpad_1/local/kaggle/santa_2017/submission/submission_1514044835.txt");
    println!("base score: {}", base_assignment.compute_score(&input).get_full_score());
    let base_assignment = LoopSolver::new(&input).optimize(&base_assignment, &Slot::create_nones());
    println!("base score: {}", base_assignment.compute_score(&input).get_full_score());

    BeamSearch {
        base_assignment: Arc::new(base_assignment),
        base_dir: Arc::new("/home/thinkpad_1/local/kaggle/santa_2017/beam".to_string()),
        input: Arc::new(input),
    }.search();
}
