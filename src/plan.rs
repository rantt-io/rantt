use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub description: String,
    pub effort: Effort,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Effort {
    pub hours: u32,
    pub days: u32,
}

pub fn get_or_create_plan<P: AsRef<Path>>(data_dir: P) -> Plan {
    let data_dir_path = data_dir.as_ref();
    if !data_dir_path.is_dir() {
        fs::create_dir_all(data_dir_path).expect("couldn't create data dir");
    }

    let plan_file = data_dir_path.join("plan.json");
    if plan_file.exists() {
        load(&plan_file)
    } else {
        let plan = default_plan();
        save(&plan_file, &plan);
        plan
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> Plan {
    let mut f = File::open(path).expect("couldn't open plan path");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("couldn't read plan data");
    serde_json::from_str(&contents).expect("couldn't deserialize plan")
}

pub fn save<P: AsRef<Path>>(path: P, plan: &Plan) {
    let mut f = File::create(path).expect("couldn't create plan file");
    write!(
        f,
        "{}",
        serde_json::to_string_pretty(plan).expect("couldn't serialize plan")
    )
    .expect("couldn't write path contents");
}

fn default_plan() -> Plan {
    Plan { tasks: vec![] }
}
