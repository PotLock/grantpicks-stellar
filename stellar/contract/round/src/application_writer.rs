use loam_sdk::soroban_sdk::{Env, Vec};

use crate::{data_type::ProjectApplication, storage_key::ContractKey};

pub fn read_application_number(env: &Env) -> u128 {
    let key = ContractKey::ApplicationNumber;
    match env.storage().persistent().get(&key) {
        Some(value) => value,
        None => 0,
    }
}

pub fn increment_application_number(env: &Env) -> u128 {
    let key = ContractKey::ApplicationNumber;
    let application_number = read_application_number(env) + 1;
    env.storage().persistent().set(&key, &application_number);
    application_number
}

pub fn add_application(env: &Env, application: ProjectApplication) {
    let mut applications = read_application(env);
    applications.push_back(application);
    write_application(env, applications);
}

pub fn read_application(env: &Env) -> Vec<ProjectApplication> {
    let key = ContractKey::ProjectApplicants;
    match env.storage().persistent().get(&key) {
        Some(value) => value,
        None => Vec::new(env),
    }
}

pub fn write_application(env: &Env, applications: Vec<ProjectApplication>) {
    let key = ContractKey::ProjectApplicants;
    env.storage().persistent().set(&key, &applications);
}

pub fn update_application(env: &Env, application: ProjectApplication) {
    let mut applications = read_application(env);
    let index = applications
        .iter()
        .position(|x| x.applicant == application.applicant)
        .unwrap();
    let index_u32: u32 = index.try_into().unwrap();
    applications.set(index_u32, application);
    write_application(env, applications);
}

pub fn find_applications(
    env: &Env,
    skip: Option<u64>,
    limit: Option<u64>,
) -> Vec<ProjectApplication> {
    let applications = read_application(env);
    let skip = skip.unwrap_or(0) as usize;
    let limit = limit.unwrap_or(10) as usize;
    assert!(limit <= 20, "limit should be less than or equal to 20");
    let mut found_applications: Vec<ProjectApplication> = Vec::new(env);

    applications
        .iter()
        .skip(skip)
        .take(limit)
        .for_each(|application| {
            found_applications.push_back(application.clone());
        });

    found_applications
}

pub fn get_application(env: &Env, project_id: u128) -> Option<ProjectApplication> {
    let applications = read_application(env);
    let application = applications
        .iter()
        .find(|application| application.project_id == project_id);

    application
}

pub fn get_application_by_id(env: &Env, application_id: u128) -> Option<ProjectApplication> {
    let applications = read_application(env);
    let skip = application_id - 1;
    let application = applications
        .iter()
        .skip(skip as usize)
        .take(1)
        .find(|application| application.application_id == application_id);

    application
}