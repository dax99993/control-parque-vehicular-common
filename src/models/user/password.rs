use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Validate)]
pub struct CambiarMiPassword {
    #[validate(length(min = 6, max = 255, message = "Contraseña invalida"))]
    pub password_actual: String,
    #[validate(length(min = 6, max = 255, message = "Contraseña invalida"))]
    pub nuevo_password: String,
    #[validate(
        length(min = 6, max = 255, message = "Contraseña invalida"),
        must_match(other = "nuevo_password", message = "Contraseñas no coinciden")
    )]
    pub re_nuevo_password: String,
}


#[derive(Debug, Clone, Default, Deserialize, Serialize, Validate)]
pub struct CambiarPassword {
    #[validate(length(min = 6, max = 255, message = "Contraseña invalida"))]
    pub nuevo_password: String,
    #[validate(
        length(min = 6, max = 255, message = "Contraseña invalida"),
        must_match(other = "nuevo_password", message = "Contraseñas no coinciden")
    )]
    pub re_nuevo_password: String,
}
