use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[cfg(feature = "backend")]
#[derive(sqlx::Type)]
#[sqlx(type_name = "estado_vehiculo", rename_all="lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum EstadoPeticion {
    #[default]
    #[strum(ascii_case_insensitive)]
    Pendiente,
    #[strum(ascii_case_insensitive)]
    Rechazado,
    #[strum(ascii_case_insensitive)]
    Aprobado,
    #[strum(ascii_case_insensitive)]
    Finalizado,
}


#[cfg(feature = "frontend")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum EstadoPeticion {
    #[default]
    #[strum(ascii_case_insensitive)]
    Pendiente,
    #[strum(ascii_case_insensitive)]
    Rechazado,
    #[strum(ascii_case_insensitive)]
    Aprobado,
    #[strum(ascii_case_insensitive)]
    Finalizado,
}


#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct Peticion {
    //id
    pub peticion_id: Uuid,
    //referencias
    pub usuario_id: Option<Uuid>,
    pub vehiculo_id: Option<Uuid>,
    //campos
    pub estado: EstadoPeticion,
    pub inicio: NaiveDateTime,
    pub finalizo: NaiveDateTime,
    pub actividad_descripcion: String,
    pub actividad_comentario: String,
    pub kilometraje_inicial: i32,
    pub kilometraje_final: i32,
    //imagenes
    pub usuario_licencia_imagen: String,
    //pub vehiculo_imagen: String,
    //pub gasolina_imagen: String,
    pub vehiculo_imagen: Option<String>,
    pub gasolina_imagen: Option<String>,
    //fechas
    pub creado_en: NaiveDateTime,
    pub modificado_en: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct NuevaPeticion {
    //referencias
    //pub usuario_id: Uuid, //Proporcionado por la informacion del api
    //pub vehiculo_id: Uuid,
    //campos
    //pub estado: String, //Por defecto 
    pub actividad_descripcion: String,
    //pub actividad_commentario: String,
    //tiempo
    pub inicio: NaiveDateTime,
    pub finalizo: NaiveDateTime,
    //kilometraje
    #[validate(
        range(min = 0, message = "Kilometraje inicial debe ser positivo"),
    )]
    pub kilometraje_inicial: i32,
    //#[validate(
    //    length(min = 0, message = "Kilometraje final debe ser positivo"),
    //)]
    //pub kilometraje_final: i32,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct EditarPeticion {
    //referencias
    //campos
    //pub estado: String, //Por defecto 
    pub actividad_descripcion: String,
    //pub actividad_commentario: String,
    //tiempo
    pub inicio: NaiveDateTime,
    pub finalizo: NaiveDateTime,
    //kilometraje
    #[validate(
        range(min = 0, message = "Kilometraje inicial debe ser positivo"),
    )]
    pub kilometraje_inicial: i32,
    //#[validate(
    //    length(min = 0, message = "Kilometraje final debe ser positivo"),
    //)]
    //pub kilometraje_final: i32,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct FinalizarPeticion {
    //campos
    pub actividad_descripcion: String,
    pub actividad_commentario: String,
    //tiempo
    //pub finalizo: NaiveDateTime,
    #[validate(
        range(min = 0, message = "Kilometraje final debe ser positivo"),
    )]
    pub kilometraje_final: i32,
    //imagenes
    //pub gasolina_imagen: String,
}
