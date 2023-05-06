use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use validator::{Validate, ValidationError};

use std::{borrow::Cow, fmt::Display};

use super::ActualizaVehiculo;


#[derive(Debug, Deserialize)]
pub struct FilterQueryVehicule {
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub año: Option<i16>,
    pub numero_placa: Option<String>,
    //pub estado: Option<EstadoVehiculo>,
    pub estado: Option<String>,
    pub activo: Option<bool>,
    // Will add more after verifing working

    // pages
    pub pagina: Option<i64>,
    pub limite: Option<i64>,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Vehiculo {
    pub vehiculo_id: Uuid,
    pub marca: String,
    pub modelo: String,
    pub año: i16,
    pub nombre_economico: String,
    pub numero_placa: String,
    pub numero_tarjeta: String,
    //pub estado: String,
    pub estado: EstadoVehiculo,
    pub activo: bool,
    pub imagen: String,
    pub creado_en: NaiveDateTime,
    pub modificado_en: NaiveDateTime,
}

impl Vehiculo {
    pub fn esta_activo(&self) -> bool {
        self.activo
    }

    pub fn activo_a_palabra(&self) -> String {
        if self.activo {
            return "si".to_string();
        } else { 
            return "no".to_string()
        };
    }

    pub fn actualizar(&mut self, actualizacion: ActualizaVehiculo) {
        if actualizacion.marca.is_some() {
            self.marca = actualizacion.marca.unwrap();
        }
        if actualizacion.modelo.is_some() {
            self.modelo = actualizacion.modelo.unwrap();
        }
        if actualizacion.año.is_some() {
            self.año= actualizacion.año.unwrap();
        }
        if actualizacion.numero_placa.is_some() {
            self.numero_placa= actualizacion.numero_placa.unwrap();
        }
        if actualizacion.nombre_economico.is_some() {
            self.nombre_economico= actualizacion.nombre_economico.unwrap();
        }
        if actualizacion.numero_tarjeta.is_some() {
            self.numero_tarjeta= actualizacion.numero_tarjeta.unwrap();
        }
        if actualizacion.estado.is_some() {
            self.estado= actualizacion.estado.unwrap();
        }
        if actualizacion.activo.is_some() {
            self.activo= actualizacion.activo.unwrap();
        }
    }

    pub fn imagen_url(&self, base_url: &str) -> String {
        format!("{base_url}api/images?filename={}", self.imagen)
    }
}

#[cfg(feature = "backend")]
#[derive(sqlx::Type)]
#[sqlx(type_name = "estado_vehiculo", rename_all="lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum EstadoVehiculo {
    #[default]
    #[strum(ascii_case_insensitive)]
    Disponible,
    #[strum(ascii_case_insensitive)]
    Ocupado,
    #[strum(ascii_case_insensitive)]
    Mantenimiento,
}


#[cfg(feature = "frontend")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum EstadoVehiculo {
    #[default]
    #[strum(ascii_case_insensitive)]
    Disponible,
    #[strum(ascii_case_insensitive)]
    Ocupado,
    #[strum(ascii_case_insensitive)]
    Mantenimiento,
}



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VehiculoFiltrado{
    pub vehiculo_id: Uuid,
    pub marca: String,
    pub modelo: String,
    pub año: i16,
    pub numero_placa: String,
    pub nombre_economico: String,
    pub numero_tarjeta: String,
    //pub status: String,
    //pub active: bool,
    pub imagen: String,
    //pub created_at: NaiveDateTime,
    //pub updated_at: NaiveDateTime,
}

impl From<Vehiculo> for VehiculoFiltrado {
    fn from(v: Vehiculo) -> Self {
        Self { 
            vehiculo_id: v.vehiculo_id,
            marca: v.marca,
            modelo: v.modelo,
            año: v.año,
            numero_placa: v.numero_placa,
            nombre_economico: v.nombre_economico,
            numero_tarjeta: v.numero_tarjeta,
            imagen: v.imagen,
        }
    }
}
