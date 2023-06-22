use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Validate)]
pub struct Departamento {
    pub id: i32,
    pub nombre: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct NuevoDepartamento {
    #[validate(
        length(min = 1, message = "Nombre requerido"),
    )]
    pub nombre: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct ActualizaDepartamento {
    #[validate(
        length(min = 1, message = "Nombre requerido"),
    )]
    pub nombre: String,
}
