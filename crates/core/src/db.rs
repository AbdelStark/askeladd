use rusqlite::{params, Connection, Result};

use crate::dvm::types::{FibonnacciProvingRequest, FibonnacciProvingResponse};

pub struct Database {
    conn: Connection,
}

#[derive(Debug)]
pub enum RequestStatus {
    Pending,
    Completed,
    Failed,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.create_table()?;
        Ok(db)
    }

    fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS requests (
                id TEXT PRIMARY KEY,
                request_json TEXT NOT NULL,
                response_json TEXT,
                status TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(())
    }

    // pub fn insert_request(&self, job_id: &str, request: &FibonnacciProvingRequest) -> Result<()>
    // {
    pub fn insert_request(&self, job_id: &str, request: &serde_json::Value) -> Result<()> {
        let request_json = serde_json::to_string(request).unwrap();
        self.conn.execute(
            "INSERT INTO requests (id, request_json, status) VALUES (?1, ?2, ?3)",
            params![job_id, request_json, RequestStatus::Pending.to_string()],
        )?;
        Ok(())
    }

    pub fn update_request(
        &self,
        request_id: &str,
        response: Option<&serde_json::Value>,
        status: RequestStatus,
    ) -> Result<()> {
        let response_json = match response {
            Some(response) => serde_json::to_string(response).unwrap(),
            None => "".to_string(),
        };
        self.conn.execute(
            "UPDATE requests SET response_json = ?1, status = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
            params![response_json, status.to_string(), request_id],
        )?;
        Ok(())
    }

    pub fn get_request_status(&self, request_id: &str) -> Result<Option<RequestStatus>> {
        let mut stmt = self
            .conn
            .prepare("SELECT status FROM requests WHERE id = ?1")?;
        let mut rows = stmt.query(params![request_id])?;

        if let Some(row) = rows.next()? {
            let status: String = row.get(0)?;
            Ok(Some(status.parse().unwrap()))
        } else {
            Ok(None)
        }
    }
}

impl ToString for RequestStatus {
    fn to_string(&self) -> String {
        match self {
            RequestStatus::Pending => "Pending".to_string(),
            RequestStatus::Completed => "Completed".to_string(),
            RequestStatus::Failed => "Failed".to_string(),
        }
    }
}

impl std::str::FromStr for RequestStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(RequestStatus::Pending),
            "Completed" => Ok(RequestStatus::Completed),
            "Failed" => Ok(RequestStatus::Failed),
            _ => Err(()),
        }
    }
}
