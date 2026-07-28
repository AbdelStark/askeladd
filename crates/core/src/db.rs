//! SQLite job ledger.
//!
//! Provers see the same job event many times — relays gossip aggressively.
//! The ledger makes completion sticky: a job marked `Completed` is never
//! executed twice, and a `Failed` job can be retried deliberately.

use std::fmt;
use std::str::FromStr;

use rusqlite::{params, Connection, Result};

/// Lifecycle of a proving request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Pending,
    Completed,
    Failed,
}

impl fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RequestStatus::Pending => "Pending",
            RequestStatus::Completed => "Completed",
            RequestStatus::Failed => "Failed",
        };
        f.write_str(s)
    }
}

/// Error returned when parsing an unknown [`RequestStatus`] string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRequestStatusError(pub String);

impl fmt::Display for ParseRequestStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown request status: {:?}", self.0)
    }
}

impl std::error::Error for ParseRequestStatusError {}

impl FromStr for RequestStatus {
    type Err = ParseRequestStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(RequestStatus::Pending),
            "Completed" => Ok(RequestStatus::Completed),
            "Failed" => Ok(RequestStatus::Failed),
            other => Err(ParseRequestStatusError(other.to_owned())),
        }
    }
}

/// Persistent store for job states, keyed by Nostr event ID.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens (creating if needed) the ledger at `path` and ensures the schema exists.
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        // NOTE: rusqlite's `execute` runs a single statement only; schema setup
        // must use `execute_batch`.
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS requests (
                id TEXT PRIMARY KEY,
                request_json TEXT NOT NULL,
                response_json TEXT,
                status TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS stwo_prover_launched (
                id TEXT PRIMARY KEY,
                request_json TEXT NOT NULL,
                status TEXT NOT NULL,
                program TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
        )?;
        Ok(())
    }

    /// Records a newly seen request as `Pending`.
    pub fn insert_request(&self, job_id: &str, request: &serde_json::Value) -> Result<()> {
        let request_json =
            serde_json::to_string(request).expect("serializing a serde_json::Value cannot fail");
        self.conn.execute(
            "INSERT INTO requests (id, request_json, status) VALUES (?1, ?2, ?3)",
            params![job_id, request_json, RequestStatus::Pending.to_string()],
        )?;
        Ok(())
    }

    /// Stores the response (if any) and final status of a request.
    pub fn update_request(
        &self,
        request_id: &str,
        response: Option<&serde_json::Value>,
        status: RequestStatus,
    ) -> Result<()> {
        let response_json = match response {
            Some(response) => serde_json::to_string(response)
                .expect("serializing a serde_json::Value cannot fail"),
            None => String::new(),
        };
        self.conn.execute(
            "UPDATE requests SET response_json = ?1, status = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
            params![response_json, status.to_string(), request_id],
        )?;
        Ok(())
    }

    /// Returns the stored status of a request, or `None` if it was never seen.
    pub fn get_request_status(&self, request_id: &str) -> Result<Option<RequestStatus>> {
        let mut stmt = self
            .conn
            .prepare("SELECT status FROM requests WHERE id = ?1")?;
        let mut rows = stmt.query(params![request_id])?;

        match rows.next()? {
            Some(row) => {
                let status: String = row.get(0)?;
                Ok(Some(status.parse().map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?))
            }
            None => Ok(None),
        }
    }

    /// Records a launched program (kind 5700 flow, experimental).
    pub fn insert_program_launched(
        &self,
        job_id: &str,
        request: &serde_json::Value,
        program: &serde_json::Value,
    ) -> Result<()> {
        let request_json =
            serde_json::to_string(request).expect("serializing a serde_json::Value cannot fail");
        let program_json =
            serde_json::to_string(program).expect("serializing a serde_json::Value cannot fail");
        self.conn.execute(
            "INSERT INTO stwo_prover_launched (id, request_json, status, program) VALUES (?1, ?2, ?3, ?4)",
            params![job_id, request_json, RequestStatus::Pending.to_string(), program_json],
        )?;
        Ok(())
    }

    /// Returns the stored status of a launched program, or `None` if never seen.
    pub fn get_program_status(&self, request_id: &str) -> Result<Option<RequestStatus>> {
        let mut stmt = self
            .conn
            .prepare("SELECT status FROM stwo_prover_launched WHERE id = ?1")?;
        let mut rows = stmt.query(params![request_id])?;

        match rows.next()? {
            Some(row) => {
                let status: String = row.get(0)?;
                Ok(Some(status.parse().map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> Database {
        Database::new(":memory:").expect("in-memory database")
    }

    #[test]
    fn unknown_request_has_no_status() {
        assert_eq!(db().get_request_status("nope").unwrap(), None);
    }

    #[test]
    fn request_lifecycle() {
        let db = db();
        let request = serde_json::json!({"log_size": 5});

        db.insert_request("job-1", &request).unwrap();
        assert_eq!(
            db.get_request_status("job-1").unwrap(),
            Some(RequestStatus::Pending)
        );

        let response = serde_json::json!({"proof": "…"});
        db.update_request("job-1", Some(&response), RequestStatus::Completed)
            .unwrap();
        assert_eq!(
            db.get_request_status("job-1").unwrap(),
            Some(RequestStatus::Completed)
        );
    }

    #[test]
    fn program_lifecycle() {
        let db = db();
        let request = serde_json::json!({"program": "fibonacci.wasm"});
        db.insert_program_launched("launch-1", &request, &request)
            .unwrap();
        assert_eq!(
            db.get_program_status("launch-1").unwrap(),
            Some(RequestStatus::Pending)
        );
    }

    #[test]
    fn status_roundtrip() {
        for status in [
            RequestStatus::Pending,
            RequestStatus::Completed,
            RequestStatus::Failed,
        ] {
            assert_eq!(status.to_string().parse(), Ok(status));
        }
        assert!("Bogus".parse::<RequestStatus>().is_err());
    }
}
