use loam_sdk::soroban_sdk::{Env, Vec};

use crate::{
    approval_writer::read_approved_projects,
    data_type::ProjectVotingResult,
    round_writer::read_round_info,
    voting_writer::{get_voting_count, read_voting_results},
};

pub fn calculate_voting_results(env: &Env) -> Vec<ProjectVotingResult> {
    let approved_projects = read_approved_projects(env);
    let voting_results = read_voting_results(env);
    let round = read_round_info(env);
    let total_voting_count = (voting_results.len() as u128) * (round.num_picks_per_voter as u128);
    let mut final_results: Vec<ProjectVotingResult> = Vec::new(env);

    approved_projects.iter().for_each(|project_id| {
        let voting_count = get_voting_count(env, project_id);
        let voting_result = ProjectVotingResult {
            project_id: project_id.clone(),
            voting_count: voting_count.clone(),
            allocation: calculate_allocation(voting_count, total_voting_count),
        };
        final_results.push_back(voting_result);
    });

    final_results
}

pub fn calculate_allocation(voting_count: u128, total_voting_count: u128) -> u128 {
    voting_count * 10000 / total_voting_count
}