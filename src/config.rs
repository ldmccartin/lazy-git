use serde_derive::{Serialize, Deserialize};

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn set_config_branch_prefix(branch_prefix: String) {
    let config = get_config();
    let new_config = LazyGitConfig {
        branch_prefix: branch_prefix.clone(),
        commit_extraction_regex: config.commit_extraction_regex.clone(),
        commit_prefix: config.commit_prefix.clone()
    };

    match confy::store(PACKAGE_NAME, None, new_config) {
        Ok(_val) => println!("Branch Prefix set to: {:?}", branch_prefix),
        Err(err) => println!("{:?}", err)
    }
}

pub fn set_config_commit_extraction_regex(commit_extraction_regex: String) {
    let config = get_config();
    let new_config = LazyGitConfig {
        branch_prefix: config.branch_prefix.clone(),
        commit_extraction_regex: commit_extraction_regex.clone(),
        commit_prefix: config.commit_prefix.clone()
    };

    match confy::store(PACKAGE_NAME, None, new_config) {
        Ok(_val) => println!("Commit Extraction Regex set to: {:?}", commit_extraction_regex),
        Err(err) => println!("{:?}", err)
    }
}

pub fn set_config_commit_prefix(commit_prefix: String) {
    let config = get_config();
    let new_config = LazyGitConfig {
        branch_prefix: config.branch_prefix.clone(),
        commit_extraction_regex: config.commit_extraction_regex.clone(),
        commit_prefix: commit_prefix.clone()
    };

    match confy::store(PACKAGE_NAME, None, new_config) {
        Ok(_val) => println!("Commit Prefix set to: {:?}", commit_prefix),
        Err(err) => println!("{:?}", err)
    }
}

pub fn display_config() {
    println!("{:?}", get_config());
}

pub fn get_config() -> LazyGitConfig {
    return confy::load(PACKAGE_NAME, None).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LazyGitConfig {
    pub branch_prefix: String,
    pub commit_extraction_regex: String,
    pub commit_prefix: String
}