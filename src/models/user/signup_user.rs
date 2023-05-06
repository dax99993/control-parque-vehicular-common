/*
use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use chrono::NaiveDateTime;
*/

//#[derive(Debug, Clone, Default, Deserialize, Serialize, Validate)]
#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct SignupUser {
    #[validate(length(min = 1, max = 255, message = "Nombre invalido"))]
    pub first_name: String,
    #[validate(length(min = 1, max = 255, message = "Apellidos invalidos"))]
    pub last_name: String,
    #[validate(
        length(min = 1, message = "Correo electronico requerido"),
        email(message = "Correo electronico invalido")
    )]
    pub email: String,
    #[validate(length(min = 6, max = 255, message = "Contraseña invalida"))]
    pub password: String,
    //pub password: Secret<String>,
    #[validate(
        must_match(other = "password", message = "Contraseñas no coinciden")
    )]
    pub re_password: String,
    //pub re_password: Secret<String>,
}
