use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use rocksdb::{DB, Options};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sindri::client::SindriClient;
use uuid::Uuid;

const DB_PATH: &str = "./prove_jobs_db";

#[derive(Serialize, Deserialize, Debug)]
struct JobRecord {
    timestamp: DateTime<Utc>,
    proof_id: String, // The Sindri Proof Identifier
    status: String,  // The status of proof generation
    x: i32,
    y: i32,
}

// Deserialize from a slice stored in the DB
impl JobRecord {
    fn from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(slice)
    }
}

// Pretty printer
impl std::fmt::Display for JobRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "requested: {:12}   proof_id: {:40} status: {:12} x: {:2}  y: {:2}",
            self.timestamp.format("%H:%M:%S"),
            self.proof_id,
            self.status,
            self.x,
            self.y
        )
    }
}

// Pretty printer
fn display_db_rows(db: &DB) {
    println!("\n\nState at {}", Utc::now());
    let iter = db.iterator(rocksdb::IteratorMode::Start);
    for result in iter {
        let (_, value) = result.unwrap();
        println!("{}", JobRecord::from_slice(&value).unwrap());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize RocksDB
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, DB_PATH)?;

    // Create a new Sindri client
    // Your api key is read from the SINDRI_API_KEY environment variable
    let client = SindriClient::default();

    // We will request proofs for a simple public circuit
    // For more details on this public circuit, visit https://sindri.app/z/sindri-rust/basic-demo/
    let circuit_id = "sindri-rust/basic-demo:latest";

    // Submit 6 new jobs
    for x in 0..2 {
        for y in 0..2 {
            // When x == y, we expect proof generation to fail!

            let local_uuid = Uuid::new_v4(); 

            // We can send our local UUID with the proof request as extra protection
            // (Making the field available from the Sindri API proof details)
            let metadata = HashMap::from([("id".to_string(), local_uuid.to_string())]);

            let proof_info = client.request_proof(
                circuit_id, 
                json!({"x": x, "y": y}), 
                Some(metadata), // Send our local UUID with the proof 
                None, 
                None
            ).await.unwrap();

            // Store the proof in our database
            db.put(local_uuid.as_bytes(), serde_json::to_string(&JobRecord {
                timestamp: Utc::now(),
                proof_id: proof_info.proof_id,
                status: proof_info.status.to_string(),
                x,
                y,
            })?.as_bytes())?;
        }
    }

    display_db_rows(&db);

    // A few seconds later, update the status of every proof in our DB
    // This will update status of proofs from the current run and previous runs still in the DB
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let iter = db.iterator(rocksdb::IteratorMode::Start);
    for result in iter {
        let (key, value) = result.unwrap();
        let mut record = JobRecord::from_slice(&value).unwrap();
        let updated_details = client.get_proof(&record.proof_id, Some(false), Some(false), Some(false)).await.unwrap();

        record.status = updated_details.status.to_string();
        db.put(key, serde_json::to_string(&record)?.as_bytes())?;
    }

    display_db_rows(&db);

    Ok(())
}
