use serde::{Serialize, Deserialize};
use validator::Validate;
use super::UsuarioRol;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct ActualizaUsuario {
    pub nombres: Option<String>,
    pub apellidos: Option<String>,
    pub email: Option<String>,
    pub numero_empleado: Option<i16>,
    pub activo: Option<bool>,
    pub verificado: Option<bool>,
    //pub departamento: Option<i32>,
    pub departamento: Option<String>,
    //pub rol: Option<String>,
    pub rol: Option<UsuarioRol>,
}
