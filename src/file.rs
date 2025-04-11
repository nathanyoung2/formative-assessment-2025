use crate::birds::{BirdTree, Node};
use serde::{Deserialize, Serialize};

use std::fs::{self, File};
use std::io::Write;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirdData {
    pub parent_nodes: Vec<String>,
    pub name: String,
    pub common_name: String,
}

/// Load data from json and deserialize it into BirdData.
pub fn load_to_tree(tree: &mut BirdTree) {
    let json = fs::read_to_string("birdData.json").expect("Could not read from file");
    let birds =
        serde_json::from_str::<Vec<BirdData>>(&json).expect("Json is formatted incorrectly");

    for bird in birds.iter() {
        tree.insert_data(bird);
    }
}

/// Get a bird data structure from a bird so that it can be saved to json.
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

/// Save an entire tree to json.
pub fn save_tree(tree: &BirdTree) {
    let mut data = vec![];

    // get bird data for each bird and push it to data accumulator
    for node in tree.direct_parents.iter() {
        for child in node.children().unwrap().borrow().iter() {
            data.push(bird_data_from_bird(Rc::clone(&child)));
        }
    }

    let mut file = File::open("birdData.json").unwrap();
    let json = serde_json::to_string(&data).unwrap();
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}
