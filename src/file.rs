use crate::birds::{BirdTree, Node};
use serde::{Deserialize, Serialize};

use std::fs;
use std::rc::Rc;

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

fn bird_data_from_bird(bird: Rc<Node>) -> BirdData {
    let parent_nodes = bird
        .full_scientific_name()
        .unwrap()
        .split(" ")
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();

    BirdData {
        parent_nodes,
        common_name: bird.name().to_string(),
        name: bird.scientific_name().to_string(),
    }
}

pub fn save_tree(tree: &BirdTree) {
    let mut data = vec![];
    for node in tree.direct_parents.iter() {
        for child in node.children().unwrap().borrow().iter() {
            data.push(bird_data_from_bird(Rc::clone(&child)));
        }
    }
}
