#![allow(dead_code)]
use chrono::{DateTime, Utc};
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum Status {
    Processing,
    Success,
    Failure,
}

#[derive(JsonSchema)]
struct ActionLog {
    action: String,
    status: Status,
    timestamp: DateTime<Utc>,
}

#[derive(JsonSchema)]
struct InputLog {
    input: String,
    status: Status,
    timestamp: DateTime<Utc>,
    actions: Vec<ActionLog>,
}

fn write_schema(dir: &std::path::Path, name: &str, schema: &RootSchema) -> std::io::Result<()> {
    let output = serde_json::to_string_pretty(schema).unwrap();
    let output_path = dir.join(format!("{}.json", name));
    std::fs::write(output_path, output)
}

fn main() -> std::io::Result<()> {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas");
    let e = std::fs::DirBuilder::new().create(&dir);
    if let Err(e) = e {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(e.into());
        }
    }

    let schema = schema_for!(ActionLog);
    write_schema(&dir, "action_log", &schema)?;

    let schema = schema_for!(InputLog);
    write_schema(&dir, "input_log", &schema)?;
    println!("Wrote schemas to {}", dir.to_string_lossy());

    Ok(())
}
