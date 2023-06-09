use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct EmployeeWorkTime {
    pub id: String,
    pub year: i32,
    pub month: i32,
    pub hours: f64,
    pub details: EmployeeWorkTimeDetails,
}

#[derive(Debug, Deserialize)]
pub struct EmployeeIDQuery {
    pub id: String,
    pub year: i32,
    pub month: i32,
}

#[derive(Debug, Serialize)]
pub struct EmployeeWorkTimeDetails {}
