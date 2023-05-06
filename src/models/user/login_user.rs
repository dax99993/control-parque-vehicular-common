use serde::{Serialize, Deserialize};
use validator::Validate;
use secrecy::Secret;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct LoginUser {
    #[validate(
        length(min = 1, message = "Correo electronico requerido"),
        email(message = "Correo electronico invalido")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Constraseña requerido"),
        length(min = 6, message = "Contraseña debe tener minimo 6 caracteres")
    )]
    pub password: String,
}


/*
#[derive(Debug, Clone, Deserialize)]
pub struct LoginUserSecret {
    pub email: String,
    pub password: Secret<String>,
}

impl From<LoginUser> for LoginUserSecret {
   fn from(value: LoginUser) -> Self {
       Self {
           email: value.email.clone(),
           password: Secret::new(value.password.clone()),
       }
   } 
}
*/
