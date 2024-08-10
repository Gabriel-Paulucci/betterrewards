use std::env::current_dir;
use std::fs::{File, read_dir, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use serde_json::{json, Value};

fn main() {
    let config_files = get_config_files();

    for path in config_files {
        let text = remove_reward(&path);
        replace(&path, &text);
    }
}

fn get_config_files() -> Vec<PathBuf> {
    let mut configs_paths = Vec::new();

    let root_config_dir = current_dir().unwrap().join("DefaultQuests").join("Quests");

    let dirs = read_dir(root_config_dir).unwrap();

    for dir in dirs {
        let paths = read_dir(dir.unwrap().path()).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            configs_paths.push(path);
        }
    }

    configs_paths
}

fn remove_reward(path: &PathBuf) -> String {
    let result = read_to_string(path).unwrap();

    let mut json = serde_json::from_str::<Value>(&result).unwrap();

    let empty = json!({});

    json["rewards:9"] = empty;
    serde_json::to_string_pretty(&json).unwrap()
}

fn replace(path: &PathBuf, text: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}