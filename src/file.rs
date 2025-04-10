use crate::birds::BirdTree;
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirdData {
    pub parent_nodes: Vec<String>,
    pub name: String,
    pub common_name: String,
}

pub fn load_to_tree(tree: &mut BirdTree) {
    let json = fs::read_to_string("birdData.json").expect("Could not read from file");
    let birds =
        serde_json::from_str::<Vec<BirdData>>(&json).expect("Json is formatted incorrectly");

    for bird in birds.iter() {
        tree.insert_data(bird);
    }
}
