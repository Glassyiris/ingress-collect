use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MetaData {
    name: String,
    namespace: String,
}

#[derive(Serialize, Deserialize)]
struct Labs {
    application: String,
    cid: String,
    env: String,
    group: String,
    project: String,
}