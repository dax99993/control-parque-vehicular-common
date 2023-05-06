use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct User {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub employee_number: Option<i16>,
    pub active: bool,
    pub verified: bool,
    pub picture: String,
    pub department: Option<i32>,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == "admin".to_string()
    }

    pub fn is_normal(&self) -> bool {
        self.role == "normal".to_string()
    }

    /*
    pub fn update(mut self, user: UpdateUser) -> Self {
        self.first_name =  user.first_name.unwrap_or_else(|| self.first_name);
        self.last_name = user.last_name.unwrap_or_else(|| self.last_name);
        self.employee_number = user.employee_number.unwrap_or_else(|| self.employee_number); 
        self.active = user.active.unwrap_or_else(|| self.active); 
        self.verified = user.verified.unwrap_or_else(|| self.verified); 
        self.department = user.department.unwrap_or_else(|| self.department);
        self.role = user.role.unwrap_or_else(|| self.role);
        self.email = user.email.unwrap_or_else(|| self.email);
        //self.picture: user.picture,

        self
    }

    pub fn update_me(mut self, user: UpdateUserMe) -> Self {
        self.first_name =  user.first_name.unwrap_or_else(|| self.first_name);
        self.last_name = user.last_name.unwrap_or_else(|| self.last_name);
        self.employee_number = user.employee_number.unwrap_or_else(|| self.employee_number); 
        self.department = user.department.unwrap_or_else(|| self.department);
        //self.email = user.email.unwrap_or_else(|| self.email);
        //self.picture: user.picture,

        self
    }
    */
     
    pub fn get_picture_url(&self, base_url: &str) -> String {
        format!("{base_url}api/images?filename={}", self.picture)
    }

    pub fn active_to_spanish(&self) -> String {
        match self.active {
            true => "Si".to_string(),
            false => "No".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FilteredUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub employee_number: Option<i16>,
    pub department: Option<i32>,
    pub picture: String,
}
