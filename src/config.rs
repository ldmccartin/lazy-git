use serde_derive::{Serialize, Deserialize};

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn set_config(branch_prefix: String) {
    let new_config = LazyGitConfig {
        branch_prefix: branch_prefix.clone()
    };

    match confy::store(PACKAGE_NAME, None, new_config) {
        Ok(_val) => println!("Branch prefix set to: {:?}", branch_prefix),
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
}