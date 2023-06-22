use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use chrono::NaiveDateTime;

use super::{update_user::ActualizaUsuario, update_user_me::ActualizaMiUsuario};

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

/*
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UsuarioFiltrado {
    pub nombres: String,
    pub apellidos: String,
    pub email: String,
    pub numero_empleado: Option<i16>,
    pub departamento: Option<i32>,
    pub imagen: String,
}

impl From<Usuario> for UsuarioFiltrado {
    fn from(value: Usuario) -> Self {
        Self {
            nombres: value.nombres,
            apellidos: value.apellidos,
            email: value.email,
            numero_empleado: value.numero_empleado,
            departamento: value.departamento,
            imagen: value.imagen,
        }
    }
}
*/


#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Usuario {
    pub usuario_id: Uuid,
    pub nombres: String,
    pub apellidos: String,
    pub email: String,
    pub password_hash: String,
    pub numero_empleado: Option<i16>,
    pub activo: bool,
    pub verificado: bool,
    pub imagen: String,
    //pub departamento: Option<i32>,
    pub departamento: String,
    pub rol: UsuarioRol,
    pub creado_en: NaiveDateTime,
    pub modificado_en: NaiveDateTime,
}

impl Usuario {
    pub fn es_admin(&self) -> bool {
        self.rol == UsuarioRol::Admin
    }

    pub fn es_normal(&self) -> bool {
        self.rol == UsuarioRol::Normal
    }

    
    pub fn actualizar(&mut self, usuario: ActualizaUsuario) {
        if usuario.nombres.is_some() {
            self.nombres=  usuario.nombres.unwrap();
        }
        if usuario.apellidos.is_some() {
            self.apellidos = usuario.apellidos.unwrap();
        }
        if usuario.numero_empleado.is_some() {
            self.numero_empleado = usuario.numero_empleado;
        }
        if usuario.activo.is_some() {
            self.activo = usuario.activo.unwrap();
        }
        if usuario.verificado.is_some() {
            self.verificado = usuario.verificado.unwrap();
        }
        if usuario.departamento.is_some() {
            self.departamento = usuario.departamento.unwrap();
        }
        if usuario.rol.is_some() {
            self.rol = usuario.rol.unwrap();
        }
        if usuario.email.is_some() {
            self.email = usuario.email.unwrap();
        }
    }

    
    pub fn actualizar_me(&mut self, usuario: ActualizaMiUsuario) {
        if usuario.nombres.is_some() {
            self.nombres=  usuario.nombres.unwrap();
        }
        if usuario.apellidos.is_some() {
            self.apellidos = usuario.apellidos.unwrap();
        }
        if usuario.numero_empleado.is_some() {
            self.numero_empleado = usuario.numero_empleado;
        }
        if usuario.departamento.is_some() {
            self.departamento = usuario.departamento.unwrap();
        }
    }
     
    pub fn imagen_url(&self, base_url: &str) -> String {
        format!("{base_url}api/images?filename={}", self.imagen)
    }

    pub fn activo_a_palabra(&self) -> String {
        match self.activo {
            true => "Si".to_string(),
            false => "No".to_string(),
        }
    }
}

#[cfg(feature = "backend")]
#[derive(sqlx::Type)]
#[sqlx(type_name = "usuario_rol", rename_all="lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum UsuarioRol{
    #[default]
    #[strum(ascii_case_insensitive)]
    Normal,
    #[strum(ascii_case_insensitive)]
    Admin,
}


#[cfg(feature = "frontend")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum UsuarioRol{
    #[default]
    #[strum(ascii_case_insensitive)]
    Normal,
    #[strum(ascii_case_insensitive)]
    Admin,
}
