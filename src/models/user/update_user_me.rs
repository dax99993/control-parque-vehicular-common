use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct UpdateUserMe {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub employee_number: Option<Option<i16>>,
    //pub email: Option<String>,
    //pub picture: Option<String>,
    pub department: Option<Option<i32>>,
}

