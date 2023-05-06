use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Validate)]
pub struct ChangePasswordMe {
    #[validate(length(min = 6, max = 255, message = "Contraseña invalida"))]
    pub current_password: String,
    #[validate(length(min = 6, max = 255, message = "Contraseña invalida"))]
    pub new_password: String,
    #[validate(
        must_match(other = "new_password", message = "Contraseñas no coinciden")
    )]
    pub re_new_password: String,
}
