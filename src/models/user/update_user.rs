use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub employee_number: Option<i16>,
    pub active: Option<bool>,
    pub verified: Option<bool>,
    pub department: Option<i32>,
    pub role: Option<String>,
}
