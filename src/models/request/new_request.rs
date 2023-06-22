use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct VehiculeRequest {
    //id
    pub peticion_id: Uuid,
    //referencias
    pub usuario_id: Uuid,
    pub vehiculo_id: Uuid,
    //campos
    pub estado: String,
    pub actividad_descripcion: String,
    pub actividad_commentario: String,
    //pub kilometraje_inicial: u32,
    pub kilometraje_final: u32,
    //imagenes
    pub licencia_imagen: String,
    pub vehiculo_imagen: String,
    pub gasolina_imagen: String,
    //fechas
    pub creado_en: NaiveDateTime,
    pub modificado_en: NaiveDateTime,
}


#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct NewVehiculeRequest {
    //referencias
    pub usuario_id: Uuid,
    pub vehiculo_id: Uuid,
    //campos
    pub estado: String,
    pub actividad_descripcion: String,
    pub actividad_commentario: String,
    //tiempo
    //pub creado_en: NaiveDateTime,
    //pub modificado_en: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, Validate)]
pub struct UpdateVehiculeRequest {
    #[validate(
        length(min = 1, message = "Descripcion de la actividad requerida"),
    )]
    pub actividad_descripcion: String,
    pub actividad_commentario: String,
    
    //#[validate(
    //    length(min = 1, message = "Descripcion de la actividad requerida"),
    //)]
    pub kilometraje_final: u32,
    //pub kilometraje_inicial: u32,
}
