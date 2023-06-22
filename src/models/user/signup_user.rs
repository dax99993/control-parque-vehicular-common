use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Validate)]
pub struct SignupUsuario {
    #[validate(length(min = 1, max = 255, message = "Nombre invalido"))]
    pub nombres: String,
    #[validate(length(min = 1, max = 255, message = "Apellidos invalidos"))]
    pub apellidos: String,
    #[validate(
        length(min = 1, message = "Correo electronico requerido"),
        email(message = "Correo electronico invalido")
    )]
    pub email: String,
    #[validate(length(min = 6, max = 255, message = "Contraseña invalida"))]
    pub password: String,
    #[validate(
        length(min = 6, max = 255, message = "Contraseña invalida"),
        must_match(other = "password", message = "Contraseñas no coinciden")
    )]
    pub re_password: String,
}
