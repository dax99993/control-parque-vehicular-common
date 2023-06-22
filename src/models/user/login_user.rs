use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct LoginUsuario {
    #[validate(
        length(min = 1, message = "Correo electronico requerido"),
        email(message = "Correo electronico invalido")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Constraseña requerida"),
        length(min = 6, message = "Contraseña debe tener minimo 6 caracteres")
    )]
    pub password: String,
}
