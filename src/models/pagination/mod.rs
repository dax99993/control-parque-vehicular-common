use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub items_per_page: Option<i64>,
}


impl Default for Pagination{
   fn default() -> Self {
       Self { page: Some(1), items_per_page: Some(5) }
   } 
}

impl Pagination {
    pub fn to_query(&self) -> String { 
        format!("page={}&items_per_page={}", self.page.unwrap(), self.items_per_page.unwrap())
    }
}

